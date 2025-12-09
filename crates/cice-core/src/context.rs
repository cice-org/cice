use crate::action::{Action, ActionParams};
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
pub struct ContextBuilder<'task, RUNTIME: Runtime, PARAMS: ActionParams> {
    runtime: RUNTIME,
    tasks: HashMap<TaskId, Task<'task, RUNTIME, PARAMS>>,
    context_handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

impl<'task, RUNTIME: Runtime, PARAMS: ActionParams> ContextBuilder<'task, RUNTIME, PARAMS> {
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
    /// Adds a single task to the context builder
    ///
    /// # Parameters
    /// * `task_config` - Task configuration containing task name and other metadata
    /// * `action` - A reference to an implementation of the Action trait with lifetime 'task
    /// * `params` - Optional byte array parameters to pass to the task for initialization. Generally it's serialized from the `Action's` input parms and deserialized back to the `Action's` input parms.
    ///
    /// # Returns
    /// Returns a mutable reference to self, enabling method chaining
    ///
    /// # Details
    /// This method inserts the task into the internal HashMap, using the task name as the key.
    /// The task is created by wrapping the provided configuration, action, and parameters.
    pub fn add_task(
        &mut self,
        task_config: TaskConfig,
        action: &'task impl Action<RUNTIME, PARAMS>,
        params: PARAMS,
    ) -> &mut Self {
        self.tasks.insert(
            task_config.task_name.clone(),
            Task::new(task_config, action, params),
        );
        self
    }

    /// Adds multiple tasks to the context builder in batch
    ///
    /// # Parameters
    /// * `tasks` - A vector of task tuples, where each tuple contains:
    ///   - `TaskConfig`: Task configuration
    ///   - `&'task impl Action<RUNTIME>`: Reference to the action implementation
    ///   - `PARAMS`: Optional parameter byte array
    ///
    /// # Returns
    /// Returns a mutable reference to self, enabling method chaining
    ///
    /// # Details
    /// This method internally iterates through the task list and calls `add_task`
    /// for each task to add them one by one.
    pub fn add_tasks(
        &mut self,
        tasks: Vec<(TaskConfig, &'task impl Action<RUNTIME, PARAMS>, PARAMS)>,
    ) -> &mut Self {
        for task in tasks {
            self.add_task(task.0, task.1, task.2);
        }
        self
    }

    /// Builds and returns the final Context object
    ///
    /// # Returns
    /// Returns a new Context instance containing all added tasks and configurations
    ///
    /// # Details
    /// This method consumes self (takes ownership) and wraps all internal state
    /// (runtime, tasks, handler, etc.) into an `Arc<ContextInner>` to enable
    /// sharing the context across multiple locations.
    ///
    /// The built Context contains:
    /// - `runtime`: The runtime environment for task execution
    /// - `tasks`: All registered tasks stored in a HashMap
    /// - `handler`: Context handler for cancellation and message communication
    /// - `cancel_recv`: Receiver for cancellation signals
    /// - `message_sender`: Sender for broadcasting messages
    pub fn build(self) -> Context<'task, RUNTIME, PARAMS> {
        Context(Arc::new(ContextInner {
            runtime: self.runtime,
            tasks: self.tasks,
            handler: self.context_handler,
            cancel_recv: self.cancel_recv,
            message_sender: self.message_sender,
        }))
    }
}

struct ContextInner<'task, RUNTIME: Runtime, PARAMS: ActionParams> {
    runtime: RUNTIME, //TODO maybe can extract runtime out of Arc inner thus can use &mut directly
    tasks: HashMap<TaskId, Task<'task, RUNTIME, PARAMS>>,
    handler: ContextHandler,
    cancel_recv: async_channel::Receiver<()>,
    message_sender: async_channel::Sender<Message>,
}

pub struct Context<'task, RUNTIME: Runtime, PARAMS: ActionParams>(
    Arc<ContextInner<'task, RUNTIME, PARAMS>>,
);

impl<RUNTIME: Runtime, PARAMS: ActionParams> Context<'_, RUNTIME, PARAMS> {
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

    pub(crate) fn get_task(&self, id: &TaskId) -> Option<&Task<RUNTIME, PARAMS>> {
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
