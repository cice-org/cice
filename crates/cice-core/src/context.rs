use futures::future::BoxFuture;

use crate::controller::{Controller, ControllerError, ControllerId};
use crate::message::Message;
use crate::recognizer::{Recognizer, RecognizerError, RecognizerId};
use crate::resource::ResourceData;
use crate::task::{Task, TaskData, TaskError, TaskId, TaskResult};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct ControllerWrapper(Arc<ControllerWrapperInner>);

struct ControllerWrapperInner {
    controller: Box<dyn Controller>,
    config: ResourceData,
    initialized: bool,
}

impl ControllerWrapper {
    fn new(controller: Box<dyn Controller>, config: ResourceData) -> Self {
        Self(Arc::new(ControllerWrapperInner {
            controller,
            config,
            initialized: false,
        }))
    }

    pub(crate) fn get_or_init(&self) -> Result<&dyn Controller, ControllerError> {
        if self.0.initialized {
            return Ok(self.0.controller.borrow());
        } else {
            self.0.controller.as_ref().init(&self.0.config)?;
            return Ok(self.0.controller.borrow());
        }
    }

    pub(crate) fn config(&self) -> &ResourceData {
        &self.0.config
    }
}
struct RecognizerWrapperInner {
    recognizer: Box<dyn Recognizer>,
    config: ResourceData,
    initialized: bool,
}
#[derive(Clone)]
pub(crate) struct RecognizerWrapper(Arc<RecognizerWrapperInner>);

impl RecognizerWrapper {
    fn new(recognizer: Box<dyn Recognizer>, config: ResourceData) -> Self {
        Self(Arc::new(RecognizerWrapperInner {
            recognizer,
            config,
            initialized: false,
        }))
    }

    fn recognizer(&self) -> Option<&dyn Recognizer> {
        if self.0.initialized {
            return Some(self.0.recognizer.borrow());
        } else {
            return None;
        }
    }

    pub(crate) fn get_or_init(&self) -> Result<&dyn Recognizer, RecognizerError> {
        if self.0.initialized {
            return Ok(self.0.recognizer.borrow());
        } else {
            self.0.recognizer.as_ref().init(&self.0.config)?;
            return Ok(self.0.recognizer.borrow());
        }
    }

    pub(crate) fn config(&self) -> &ResourceData {
        &self.0.config
    }
}

#[derive(Clone)]
pub struct ContextHandler(Arc<ContextHandlerInner>);

struct ContextHandlerInner {
    cancel_sender: async_channel::Sender<()>,
    message_recv: async_channel::Receiver<Message>,
}

impl ContextHandler {
    pub async fn cancel(&self) -> Result<(), async_channel::SendError<()>> {
        self.0.cancel_sender.send(()).await
    }

    pub fn try_cancel(&self) -> Result<(), async_channel::TrySendError<()>> {
        self.0.cancel_sender.try_send(())
    }

    pub async fn recv(&self) -> Result<Message, async_channel::RecvError> {
        self.0.message_recv.recv().await
    }

    pub fn try_recv(&self) -> Result<Message, async_channel::TryRecvError> {
        self.0.message_recv.try_recv()
    }
}

pub struct ContextBuilder {
    tasks: HashMap<TaskId, Task>,
    controllers: HashMap<ControllerId, ControllerWrapper>,
    reconizers: HashMap<RecognizerId, RecognizerWrapper>,
    context_handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        let (cancel_send, cancel_recv) = async_channel::bounded(1); //Cancel signal should be sent only once
        let (message_send, message_recv) = async_channel::bounded(20);
        Self {
            tasks: HashMap::new(),
            controllers: HashMap::new(),
            reconizers: HashMap::new(),
            context_handler: ContextHandler(Arc::new(ContextHandlerInner {
                cancel_sender: cancel_send,
                message_recv: message_recv,
            })),
            cancel_recv,
            message_sender: message_send,
        }
    }
    pub fn add_task<T: TaskData>(&mut self, task_data: T) -> &mut Self {
        self.tasks
            .insert(task_data.base_data().task_name, task_data.into());
        self
    }

    pub fn add_tasks<T: TaskData>(&mut self, tasks: Vec<T>) -> &mut Self {
        for task in tasks {
            self.add_task(task);
        }
        self
    }

    /// # Params
    /// - controller: (controller: Box<dyn Controller>, controller_config: ResourceData) controller and its config resource data
    pub fn add_controller(&mut self, controller: (Box<dyn Controller>, ResourceData)) -> &mut Self {
        self.controllers.insert(
            controller.0.name(),
            ControllerWrapper::new(controller.0, controller.1),
        );
        self
    }

    /// # Prams
    /// - controllers: Vec<Box<dyn Controller>> controllers and their config resource data
    pub fn add_controllers(
        &mut self,
        controllers: Vec<(Box<dyn Controller>, ResourceData)>,
    ) -> &mut Self {
        for controller in controllers {
            self.add_controller(controller);
        }
        self
    }

    pub fn add_recognizer(&mut self, recognizer: (Box<dyn Recognizer>, ResourceData)) -> &mut Self {
        self.reconizers.insert(
            recognizer.0.name(),
            RecognizerWrapper::new(recognizer.0, recognizer.1),
        );
        self
    }

    pub fn add_recognizers(
        &mut self,
        recognizers: Vec<(Box<dyn Recognizer>, ResourceData)>,
    ) -> &mut Self {
        for recognizer in recognizers {
            self.add_recognizer(recognizer);
        }
        self
    }

    pub fn build(self) -> Context {
        Context(Arc::new(ContextInner {
            tasks: self.tasks,
            controllers: self.controllers,
            reconizers: self.reconizers,
            handler: self.context_handler,
            cancel_recv: self.cancel_recv,
            message_sender: self.message_sender,
        }))
    }
}

struct ContextInner {
    tasks: HashMap<TaskId, Task>,
    controllers: HashMap<ControllerId, ControllerWrapper>,
    reconizers: HashMap<RecognizerId, RecognizerWrapper>,
    handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

pub struct Context(Arc<ContextInner>);

impl Context {
    pub async fn run(&self, entry: TaskId) -> Result<TaskResult, TaskError> {
        if let Some(task) = self.0.tasks.get(&entry) {
            let mut task_res = task.run_with_context(self).await;
            while let Ok(ref res) = task_res {
                match res {
                    TaskResult::Success { id } => {
                        task_res = self.get_task(&id).unwrap().run_with_context(self).await;
                        continue;
                    }
                    TaskResult::NoPendingTask => return Ok(TaskResult::NoPendingTask),
                    TaskResult::TaskCancelled => return Ok(TaskResult::TaskCancelled),
                }
            }
            task_res
        } else {
            log::error!("Entry Task {entry} not found");
            return Err(TaskError::UnknownTask { id: entry });
        }
    }

    pub fn get_handler(&self) -> ContextHandler {
        self.0.handler.clone()
    }

    pub(crate) fn get_controller(&self, id: &ControllerId) -> Option<&ControllerWrapper> {
        self.0.controllers.get(id)
    }

    pub(crate) fn get_recognizer(&self, id: &RecognizerId) -> Option<&RecognizerWrapper> {
        self.0.reconizers.get(id)
    }

    pub(crate) fn get_task(&self, id: &TaskId) -> Option<&Task> {
        self.0.tasks.get(id)
    }

    pub(crate) async fn get_cancel_signal(&self) -> Result<(), async_channel::RecvError> {
        self.0.cancel_recv.recv().await
    }

    // Always drop message if channel is full in try_send_message
    pub(crate) fn try_send_message(
        &self,
        msg: Message,
    ) -> Result<(), async_channel::TrySendError<Message>> {
        self.0.message_sender.try_send(msg)
    }
}
