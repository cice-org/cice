use std::{future::Future, pin::Pin, sync::Arc};

use alloc::{string::String, vec::Vec};
use futures::{future::select_ok, FutureExt};
use snafu::Snafu;

use crate::{
    config::BaseTaskConfig,
    context::Context,
    controller::{Controller, ControllerError},
    recognizer::{CustomRecognizerError, Recognizer, RecognizerError},
    resource::ResourceData,
};

pub type TaskId = String;

#[repr(transparent)]
#[derive(Clone)]
pub struct Task(Arc<TaskInner>); //This is a readonly

pub struct TaskInner {
    base: BaseTaskConfig,
    controller_config: ResourceData,
    recognizer_config: Option<ResourceData>,
    controller_output_action: Option<ResourceData>,
    recognizer_action: ResourceData,
}

impl TaskInner {
    pub(crate) fn base(&self) -> &BaseTaskConfig {
        &self.base
    }
}

impl Task {
    async fn try_run(&self, context: &Context) -> Result<TaskResult, TaskError> {
        let controller = self.get_controller(context)?;
        let recognizer = self.get_recognizer(context)?;
        let controller_output_action = self
            .0
            .as_ref()
            .controller_output_action
            .as_ref()
            .and_then(|value| {
                recognizer.require_input().map(|mut require_value| {
                    merge(&mut require_value, value.clone());
                    require_value
                })
            })
            .ok_or(TaskError::TaskConfigError {
                reason: "missing controller output action",
            })?;
        // TODO match arms by macro
        if let Some(recognizer) = recognizer.ext_image() {
            let output = controller
                .ext_output()
                .unwrap()
                .ext_image()
                .unwrap()
                .exec(&controller_output_action)
                .await
                .map_err(Into::<ControllerError>::into)?;
            recognizer
                .exec(&self.0.recognizer_action, output)
                .await
                .map_err(Into::<RecognizerError>::into)?;
        } else {
            return Err(TaskError::RecognizerError {
                source: RecognizerError::IncompatibleRecognizer {
                    id: recognizer.name(),
                },
            });
        }
        Ok(TaskResult::Success)
    }

    pub(crate) async fn run_with_context(
        &self,
        context: &Context,
    ) -> Result<TaskResult, TaskError> {
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
                    .map(Clone::clone)
            })
            .collect();
        if tasks.is_empty() {
            return Ok(TaskResult::NoPendingTask);
        }
        select_ok(tasks.iter().map(|task| task.try_run(context).boxed())).await?;
        Ok(TaskResult::Success)
    }

    //FIXME remove this lifetime parameter once rust compiler fix it
    fn get_controller<'b, 'a: 'b>(
        &'a self,
        context: &'a Context,
    ) -> Result<&'b dyn Controller, TaskError> {
        let controller_id = &self.0.base.controller_id;
        let wrapper =
            context
                .get_controller(&controller_id)
                .ok_or(ControllerError::ControllerNotFound {
                    id: controller_id.clone(),
                })?;
        debug_assert_eq!(wrapper.config(), &self.0.controller_config);
        return wrapper.get_or_init().map_err(|e| e.into());
    }
    fn get_recognizer<'b, 'a: 'b>(
        &'a self,
        context: &'a Context,
    ) -> Result<&'b dyn Recognizer, TaskError> {
        let recognizer_id = &self.0.base.recognizer_id;
        let wrapper =
            context
                .get_recognizer(recognizer_id)
                .ok_or(ControllerError::ControllerNotFound {
                    id: recognizer_id.clone(),
                })?;
        debug_assert_eq!(wrapper.config(), &self.0.controller_config);
        return wrapper.get_or_init().map_err(|e| e.into());
    }
}

impl<T: TaskData> From<T> for Task {
    fn from(value: T) -> Self {
        Self(Arc::new(TaskInner {
            base: value.base_data(),
            controller_config: value.controller_config(),
            recognizer_config: value.recognizer_config(),
            controller_output_action: value.controller_output_action(),
            recognizer_action: value.recognizer_action(),
        }))
    }
}

pub trait TaskData {
    fn base_data(&self) -> BaseTaskConfig;
    fn controller_config(&self) -> ResourceData;
    fn recognizer_config(&self) -> Option<ResourceData>;
    fn controller_output_action(&self) -> Option<ResourceData>;
    fn recognizer_action(&self) -> ResourceData;
}

impl TaskData for ResourceData {
    fn controller_config(&self) -> ResourceData {
        todo!()
    }

    fn recognizer_config(&self) -> Option<ResourceData> {
        todo!()
    }

    fn base_data(&self) -> BaseTaskConfig {
        todo!()
    }

    fn controller_output_action(&self) -> Option<ResourceData> {
        todo!()
    }

    fn recognizer_action(&self) -> ResourceData {
        todo!()
    }
}

pub(crate) enum TaskResult {
    Success,
    NoPendingTask,
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

//TODO remove it once it's added to serde_json
fn merge(a: &mut ResourceData, b: ResourceData) {
    match (a, b) {
        (ResourceData::Object(a), ResourceData::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(ResourceData::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}

// #[cfg(test)]
// mod test {
//     use std::sync::Arc;

//     use serde_json::json;

//     use crate::{config::BaseTaskConfig, context::ContextBuilder};

//     use super::Task;

//     fn assert_send<T: Send>(_: T) {}

//     #[test]
//     fn assert_run_with_context_send() {
//         assert_send(
//             Task(Arc::new(super::TaskInner {
//                 base: BaseTaskConfig {
//                     task_name: "".to_string(),
//                     next_task: vec![],
//                     interrupt_task: vec![],
//                     controller_id: "".to_string(),
//                     recognizer_id: "".to_string(),
//                 },
//                 controller_config: json!({}),
//                 recognizer_config: Some(json!({})),
//             }))
//             .run_with_context(&ContextBuilder::new().build()),
//         );
//     }
// }
