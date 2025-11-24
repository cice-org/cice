use crate::action::Action;
use crate::message::Message;
use crate::runtime::Runtime;
use crate::task::{Task, TaskConfig, TaskError, TaskId, TaskResult};
use std::collections::HashMap;
use std::sync::Arc;

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

//could have a 'context lifetime specifier which is longer than 'task if needed
pub struct ContextBuilder<'task, RUNTIME: Runtime> {
    runtime: RUNTIME,
    tasks: HashMap<TaskId, Task<'task, RUNTIME>>,
    context_handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

impl<'task, RUNTIME: Runtime> ContextBuilder<'task, RUNTIME> {
    pub fn new(runtime: RUNTIME) -> Self {
        let (cancel_sender, cancel_recv) = async_channel::bounded(1); //Cancel signal should be sent only once
        let (message_sender, message_recv) = async_channel::bounded(20);
        Self {
            runtime,
            tasks: HashMap::new(),
            context_handler: ContextHandler(Arc::new(ContextHandlerInner {
                cancel_sender,
                message_recv,
            })),
            cancel_recv,
            message_sender,
        }
    }
    pub fn add_task(
        &mut self,
        task_config: TaskConfig,
        action: &'task impl Action<RUNTIME>,
    ) -> &mut Self {
        self.tasks.insert(
            task_config.task_name.clone(),
            Task::new(task_config, action),
        );
        self
    }

    pub fn add_tasks(
        &mut self,
        tasks: Vec<(TaskConfig, &'task impl Action<RUNTIME>)>,
    ) -> &mut Self {
        for task in tasks {
            self.add_task(task.0, task.1);
        }
        self
    }

    pub fn build(self) -> Context<'task, RUNTIME> {
        Context(Arc::new(ContextInner {
            runtime: self.runtime,
            tasks: self.tasks,
            handler: self.context_handler,
            cancel_recv: self.cancel_recv,
            message_sender: self.message_sender,
        }))
    }
}

struct ContextInner<'task, RUNTIME: Runtime> {
    runtime: RUNTIME, //TODO maybe can extract runtime out of Arc inner thus can use &mut directly
    tasks: HashMap<TaskId, Task<'task, RUNTIME>>,
    handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

pub struct Context<'task, RUNTIME: Runtime>(Arc<ContextInner<'task, RUNTIME>>);

impl<RUNTIME: Runtime> Context<'_, RUNTIME> {
    pub async fn run(&self, entry: TaskId) -> Result<TaskResult, TaskError> {
        if let Some(task) = self.0.tasks.get(&entry) {
            let mut task_res = task.run_with_context(self).await;
            while let Ok(ref res) = task_res {
                match res {
                    TaskResult::Success { id } => {
                        task_res = self.get_task(id).unwrap().run_with_context(self).await;
                        continue;
                    }
                    TaskResult::NoPendingTask => return Ok(TaskResult::NoPendingTask),
                    TaskResult::TaskCancelled => return Ok(TaskResult::TaskCancelled),
                }
            }
            task_res
        } else {
            log::error!("Entry Task {entry} not found");
            Err(TaskError::UnknownTask { id: entry })
        }
    }

    pub fn get_handler(&self) -> ContextHandler {
        self.0.handler.clone()
    }

    pub(crate) fn get_task(&self, id: &TaskId) -> Option<&Task<RUNTIME>> {
        self.0.tasks.get(id)
    }

    pub(crate) async fn get_cancel_signal(&self) -> Result<(), async_channel::RecvError> {
        self.0.cancel_recv.recv().await
    }

    pub(crate) fn get_runtime(&self) -> &RUNTIME {
        &self.0.runtime
    }

    // Always drop message if channel is full in try_send_message
    pub(crate) fn try_send_message(
        &self,
        msg: Message,
    ) -> Result<(), async_channel::TrySendError<Message>> {
        self.0.message_sender.try_send(msg)
    }
}
