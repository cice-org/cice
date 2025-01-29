use core::cell::RefCell;

use alloc::{rc::Rc, string::String, vec::Vec};

use crate::{
    config::BaseTaskConfig, context::Context, controller::Controller, recognizer::Recognizer,
    resource::ResourceData,
};

pub type TaskId = String;

#[repr(transparent)]
#[derive(Clone)]
pub struct Task(Rc<RefCell<TaskInner>>);

pub struct TaskInner {
    base: BaseTaskConfig,
    controller_config: ResourceData,
    recognizer_config: Option<ResourceData>,
}
impl Task {
    pub fn run_with_context(&self, context: &mut Context) {}
}

impl<T: TaskData> From<T> for Task {
    fn from(value: T) -> Self {
        Self(Rc::new(RefCell::new(TaskInner {
            base: value.base_data(),
            controller_config: value.controller_config(),
            recognizer_config: value.recognizer_config(),
        })))
    }
}

pub trait TaskData {
    fn base_data(&self) -> BaseTaskConfig;
    fn controller_config(&self) -> ResourceData;
    fn recognizer_config(&self) -> Option<ResourceData>;
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
}

pub enum TaskError {
    UnknownTask(TaskId),
    TaskCancelled(TaskId),
}
