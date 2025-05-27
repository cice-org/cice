use alloc::sync::Arc;

use alloc::{string::String, vec::Vec};
use futures::{select, FutureExt, StreamExt};
use snafu::Snafu;

use crate::controller::ControllerId;
use crate::message::task::TaskMessage;
use crate::message::Message;
use crate::recognizer::RecognizerId;
use crate::utils::merge;
use crate::{
    context::Context,
    controller::{Controller, ControllerError},
    recognizer::{Recognizer, RecognizerError},
    resource::ResourceData,
};

pub type TaskId = String;

#[repr(transparent)]
#[derive(Clone)]
pub struct Task(Arc<TaskInner>); //This is a readonly

pub struct TaskInner {
    base: BaseTaskConfig,
    controller_config_ext: Option<ResourceData>,
    recognizer_config_ext: Option<ResourceData>,
    controller_output_action: Option<ResourceData>,
    controller_input_action: Option<ResourceData>,
    recognizer_action: Option<ResourceData>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseTaskConfig {
    pub task_name: TaskId,
    pub next_task: Vec<TaskId>,
    pub interrupt_task: Vec<TaskId>,
    pub controller_id: ControllerId,
    pub recognizer_id: RecognizerId,
}

impl TaskInner {
    pub(crate) fn base(&self) -> &BaseTaskConfig {
        &self.base
    }
}

impl Task {
    async fn try_run(&self, context: &Context) -> Result<TaskResult, TaskError> {
        Self::send_task_message(
            context,
            TaskMessage::TryExec {
                id: self.base().task_name.clone(),
            },
        );
        let controller = self.get_controller(context)?;
        let recognizer = self.get_recognizer(context)?;
        // Reinit controller or recognizer if there is extended config in task
        if let Some(ref config) = self.0.controller_config_ext {
            self.reinit_controller(context, config, controller)?;
        }
        if let Some(ref config) = self.0.recognizer_config_ext {
            self.reinit_recognizer(context, config, recognizer)?;
        }
        //Merge Controller output_action
        let mut ext_controller_output_action = recognizer.require_input();
        let base_controller_output_action = self.0.as_ref().controller_output_action.as_ref();
        //FIXME it's an opening issue https://github.com/rust-lang/rust-clippy/issues/13185
        #[allow(clippy::manual_inspect)]
        let controller_output_action = if let Some(base_action) = base_controller_output_action {
            ext_controller_output_action.as_mut().map(|value| {
                merge(value, base_action.clone());
                value
            });
            ext_controller_output_action
        } else {
            ext_controller_output_action
        }
        .ok_or(TaskError::TaskConfigError {
            reason: "missing controller output action",
        })?;

        Self::send_task_message(
            context,
            TaskMessage::ExecController {
                task_id: self.base().task_name.clone(),
                controller_id: self.base().controller_id.clone(),
            },
        );
        let output = controller
            .exec_output(&controller_output_action)
            .await
            .map_err(Into::<ControllerError>::into)?;
        Self::send_task_message(
            context,
            TaskMessage::ExecRecognizer {
                task_id: self.base().task_name.clone(),
                recognizer_id: self.base().recognizer_id.clone(),
            },
        );
        let mut reco_result = recognizer
            .exec(self.0.recognizer_action.as_ref(), output)
            .await
            .map_err(Into::<RecognizerError>::into)?;

        if let Some(action) = self.0.controller_input_action.as_ref() {
            merge(&mut reco_result, action.clone())
        }
        controller
            .exec_input(&reco_result)
            .await
            .map_err(Into::<ControllerError>::into)?;
        Self::send_task_message(
            context,
            TaskMessage::ExecSuccess {
                id: self.base().task_name.clone(),
            },
        );

        Ok(TaskResult::Success {
            id: self.base().task_name.clone(),
        })
    }

    pub(crate) async fn run_with_context(
        &self,
        context: &Context,
    ) -> Result<TaskResult, TaskError> {
        Self::send_task_message(
            context,
            TaskMessage::Enter {
                id: self.base().task_name.clone(),
            },
        );
        let inner = self.0.as_ref();
        let tasks: Vec<Task> = inner
            .base
            .next_task
            .iter()
            .filter_map(|id| {
                context
                    .get_task(id)
                    .or_else(|| {
                        //Leaving it a log error instead of breaking running
                        log::error!("no task found for id {id}");
                        None
                    })
                    .cloned()
            })
            .collect();
        if tasks.is_empty() {
            return Ok(TaskResult::NoPendingTask);
        }
        let mut cancel_signal = context.get_cancel_signal().boxed().fuse();
        let mut task_futures = futures::stream::FuturesOrdered::new();
        tasks.iter().for_each(|task| {
            task_futures.push_back(task.try_run(context).boxed());
        });
        loop {
            select! {
                   res = task_futures.next() => match res {
                       Some(Ok(res)) => return Ok(res),
                       Some(Err(e)) => {
                           if e.is_fatal(){
                               return Err(e)
                           }else{
                               continue;
                           }
                       },
                       None => return Ok(TaskResult::NoPendingTask),
                   },
                   _ = cancel_signal => {
                       // Cancel signal received
                       return Ok(TaskResult::TaskCancelled)
                }
            }
        }
    }

    //FIXME remove this lifetime parameter once rust compiler fix it
    fn get_controller<'b, 'a: 'b>(
        &'a self,
        context: &'a Context,
    ) -> Result<&'b dyn Controller, TaskError> {
        let controller_id = &self.0.base.controller_id;
        let wrapper =
            context
                .get_controller(controller_id)
                .ok_or(ControllerError::ControllerNotFound {
                    id: controller_id.clone(),
                })?;
        wrapper.get_or_init().map_err(|e| e.into())
    }
    fn get_recognizer<'b, 'a: 'b>(
        &'a self,
        context: &'a Context,
    ) -> Result<&'b dyn Recognizer, TaskError> {
        let recognizer_id = &self.0.base.recognizer_id;
        let wrapper =
            context
                .get_recognizer(recognizer_id)
                .ok_or(RecognizerError::RecognizerNotFound {
                    id: recognizer_id.clone(),
                })?;
        wrapper.get_or_init().map_err(|e| e.into())
    }

    fn reinit_controller(
        &self,
        context: &Context,
        ext_config: &ResourceData,
        controller: &dyn Controller,
    ) -> Result<(), ControllerError> {
        let mut base_config = context
            .get_controller(&self.base().controller_id)
            .unwrap()
            .config()
            .clone();
        merge(&mut base_config, ext_config.clone());
        controller.init(&base_config)?;
        Ok(())
    }

    fn reinit_recognizer(
        &self,
        context: &Context,
        ext_config: &ResourceData,
        recognizer: &dyn Recognizer,
    ) -> Result<(), RecognizerError> {
        let mut base_config = context
            .get_recognizer(&self.base().controller_id)
            .unwrap()
            .config()
            .clone();
        merge(&mut base_config, ext_config.clone());
        recognizer.init(&base_config)?;
        Ok(())
    }

    fn base(&self) -> &BaseTaskConfig {
        self.0.as_ref().base()
    }

    fn send_task_message(context: &Context, msg: TaskMessage) {
        if let Err(e) = context.try_send_message(Message::TaskMessage(msg)) {
            log::error!("Failed to send message {e}");
        }
    }
}

impl<T: TaskData> From<T> for Task {
    fn from(value: T) -> Self {
        Self(Arc::new(TaskInner {
            base: value.base_data(),
            controller_config_ext: value.controller_config_ext(),
            recognizer_config_ext: value.recognizer_config_ext(),
            controller_output_action: value.controller_output_action_ext(),
            controller_input_action: value.controller_input_action_ext(),
            recognizer_action: value.recognizer_action(),
        }))
    }
}

pub trait TaskData {
    fn base_data(&self) -> BaseTaskConfig;
    fn controller_config_ext(&self) -> Option<ResourceData>;
    fn recognizer_config_ext(&self) -> Option<ResourceData>;
    fn controller_output_action_ext(&self) -> Option<ResourceData>;
    fn controller_input_action_ext(&self) -> Option<ResourceData>;
    fn recognizer_action(&self) -> Option<ResourceData>;
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Success { id: TaskId },
    NoPendingTask,
    TaskCancelled,
}

// struct TaskErrorWrapper(Arc<TaskError>);

// impl From<TaskError> for TaskErrorWrapper {
//     fn from(value: TaskError) -> Self {
//         Self(Arc::new(value))
//     }
// }

#[derive(Debug, Snafu)]
pub enum TaskError {
    #[snafu(display("unknown task id:{id}"))]
    UnknownTask { id: TaskId },
    #[snafu(display("task with id:{id} is cancelled"))]
    TaskCancelled { id: TaskId },
    #[snafu(display("task config error:"))]
    TaskConfigError { reason: &'static str },
    #[snafu(display("controller error:{source}"))]
    ControllerError { source: ControllerError },
    #[snafu(display("recognizer error:{source}"))]
    RecognizerError { source: RecognizerError },
}

impl From<ControllerError> for TaskError {
    fn from(value: ControllerError) -> Self {
        Self::ControllerError { source: value }
    }
}

impl From<RecognizerError> for TaskError {
    fn from(value: RecognizerError) -> Self {
        Self::RecognizerError { source: value }
    }
}

impl TaskError {
    //return whether this erro Should break running
    //TODO should extract a trait for CustomError to implement is_fatal()?
    fn is_fatal(&self) -> bool {
        match self {
            TaskError::ControllerError {
                source: ControllerError::Err { source },
            } => match source {
                crate::controller::CustomControllerError::Fatal { source: _ } => true,
                crate::controller::CustomControllerError::Common { source: _ } => false,
            },
            TaskError::RecognizerError {
                source: RecognizerError::Err { source },
            } => match source {
                crate::recognizer::CustomRecognizerError::Fatal { source: _ } => true,
                crate::recognizer::CustomRecognizerError::Common { source: _ } => false,
            },
            _ => true,
        }
    }
}
