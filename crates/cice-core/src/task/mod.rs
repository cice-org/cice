use std::time::Duration;

use alloc::sync::Arc;

use alloc::{string::String, vec::Vec};
use futures::FutureExt;
use snafu::Snafu;

use crate::action::{Action, ActionError, ActionId, ExecError, RecognizeError};
use crate::context::Context;
use crate::message::task::TaskMessage;
use crate::message::Message;
use crate::runtime::Runtime;

pub type TaskId = String;

/// Task is the basic unit of execution in the system. Each Task is associated with a specific Action
///
/// ## Excecution Flow
/// 1. Enter: The task is entered, and a message is sent to indicate this.
/// 2. Try Recognize: The task attempts to recognize its associated action.
/// 3. Try Exec: If recognition is successful, the task attempts to execute the action.
/// 4. Exec Success: If execution is successful, a success message is sent.
/// 5. Next Tasks: After successful execution, the task checks(use `recognize`) for any next tasks to execute. If there is any next task, it will be entered and goto step 1.
///
#[repr(transparent)]
pub struct Task<'task, RUNTIME: Runtime>(Arc<TaskInner<'task, RUNTIME>>);

impl<'task, RUNTIME: Runtime> Clone for Task<'task, RUNTIME> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct TaskInner<'task, RUNTIME: Runtime> {
    config: TaskConfig,
    action: &'task dyn Action<RUNTIME>,
}

pub struct TaskConfig {
    pub task_name: TaskId,
    pub action_name: ActionId,
    pub next_task: Vec<TaskId>,
    pub interrupt_task: Vec<TaskId>,
    pub timeout: Duration, //TODO required a runtime ext to support timer in no_std env
    pub max_retry: usize,
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Success { id: TaskId },
    NoPendingTask,
    TaskCancelled,
}

#[derive(Debug, Snafu)]
pub enum TaskError {
    #[snafu(display("unknown task id:{id}"))]
    UnknownTask { id: TaskId },
    #[snafu(display("controller error {source}"))]
    ActionError { source: ActionError },
    #[snafu(display("task {id} time out"))]
    TaskTimeOut { id: TaskId },
}

impl<'task, RUNTIME: Runtime> Task<'task, RUNTIME> {
    pub fn new(config: TaskConfig, action: &'task impl Action<RUNTIME>) -> Self {
        Self(Arc::new(TaskInner::new(config, action)))
    }
}

impl<'task, RUNTIME: Runtime> TaskInner<'task, RUNTIME> {
    pub fn new(config: TaskConfig, action: &'task impl Action<RUNTIME>) -> Self {
        Self { config, action }
    }
}

impl<'task, RUNTIME: Runtime> TaskInner<'task, RUNTIME> {
    pub(crate) fn config(&self) -> &TaskConfig {
        &self.config
    }
}

impl<'task, RUNTIME: Runtime> Task<'task, RUNTIME> {
    async fn try_recognize(&self, context: &Context<'task, RUNTIME>) -> Result<(), RecognizeError> {
        Self::send_task_message(
            context,
            TaskMessage::TryRecognize {
                id: self.config().task_name.clone(),
            },
        );

        self.0.action.recognize(&context.get_runtime()).await?;

        Self::send_task_message(
            context,
            TaskMessage::ExecSuccess {
                id: self.config().task_name.clone(),
            },
        );

        Ok(())
    }

    async fn try_exec(&self, context: &Context<'task, RUNTIME>) -> Result<(), ExecError> {
        Self::send_task_message(
            context,
            TaskMessage::TryExec {
                id: self.config().task_name.clone(),
            },
        );

        self.0
            .action
            .exec(&context.get_runtime())
            .await
            .map_err(|e| e.into())?;

        Self::send_task_message(
            context,
            TaskMessage::ExecSuccess {
                id: self.config().task_name.clone(),
            },
        );

        Ok(())
    }

    async fn try_run(&self, context: &Context<'task, RUNTIME>) -> Result<TaskResult, ActionError> {
        self.try_recognize(context)
            .await
            .map_err(Into::<ActionError>::into)?;
        self.try_exec(context)
            .await
            .map_err(Into::<ActionError>::into)?;
        Ok(TaskResult::Success {
            id: self.config().task_name.clone(),
        })
    }

    pub(crate) async fn run_with_context(
        &self,
        context: &Context<'task, RUNTIME>,
    ) -> Result<TaskResult, TaskError> {
        Self::send_task_message(
            context,
            TaskMessage::Enter {
                id: self.config().task_name.clone(),
            },
        );
        let inner = self.0.as_ref();
        let next_tasks: Vec<Task<RUNTIME>> = inner
            .config
            .next_task
            .iter()
            .filter_map(|id| {
                (&context.get_task(id))
                    .or_else(|| {
                        //Leaving it a log error instead of breaking running
                        log::error!("no task found for id {id}");
                        None
                    })
                    .cloned()
            })
            .collect();
        if next_tasks.is_empty() {
            return Ok(TaskResult::NoPendingTask);
        }
        let mut cancel_signal = context.get_cancel_signal().boxed().fuse();

        let mut task_futures = vec![];
        next_tasks.iter().for_each(|task| {
            task_futures.push(task.try_run(context).boxed());
        });
        let  mut select_futures = futures::future::select_ok(task_futures).fuse();
        for _retry_count in 0..self.config().max_retry {
            futures::select! {
                   res = select_futures => match res {
                        Ok((result, _remaining)) => {
                            return Ok(result);
                        },
                        Err(_e) => {
                            //Ignore
                        }
                   },
                   _ = cancel_signal => {
                       // Cancel signal received
                       return Ok(TaskResult::TaskCancelled)
                }
            }
        }
        return Err(TaskError::TaskTimeOut {
            id: self.config().task_name.clone(),
        });
    }

    fn config(&self) -> &TaskConfig {
        self.0.as_ref().config()
    }

    fn send_task_message(context: &Context<RUNTIME>, msg: TaskMessage) {
        if let Err(e) = context.try_send_message(Message::TaskMessage(msg)) {
            log::error!("Failed to send message {e}");
        }
    }
}

impl From<ActionError> for TaskError {
    fn from(source: ActionError) -> Self {
        TaskError::ActionError { source }
    }
}
