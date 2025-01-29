use crate::controller::{Controller, ControllerId};
use crate::pipeline::Pipeline;
use crate::recognizer::{Recognizer, RecognizerId};
use crate::resource::ResourceData;
use crate::task::{Task, TaskData, TaskError, TaskId};
use std::borrow::Borrow;
use std::collections::HashMap;

struct ControllerWrapper {
    controller: Box<dyn Controller>,
    config: ResourceData,
    initialized: bool,
}

impl ControllerWrapper {
    fn new(controller: Box<dyn Controller>, config: ResourceData) -> Self {
        Self {
            controller,
            config,
            initialized: false,
        }
    }

    fn controller(&self) -> Option<&dyn Controller> {
        if self.initialized {
            return Some(self.controller.borrow());
        } else {
            return None;
        }
    }
}

pub struct Context {
    pipeline: Pipeline,
    tasks: HashMap<TaskId, Task>,
    controllers: HashMap<ControllerId, ControllerWrapper>,
    reconizers: HashMap<RecognizerId, Box<dyn Recognizer>>,
}

impl Context {
    pub fn new() -> Context {
        Self {
            pipeline: Pipeline::new(),
            tasks: HashMap::new(),
            controllers: HashMap::new(),
            reconizers: HashMap::new(),
        }
    }
    pub async fn run(&self, entry: TaskId) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.get(&entry) {
            return self.pipeline.run_pipeline(task.clone(), &self).await;
        } else {
            log::error!("Entry Task {entry} not found");
            return Err(TaskError::UnknownTask(entry));
        }
    }

    pub fn insert_task<T: TaskData>(&mut self, task_data: T) {
        self.tasks
            .insert(task_data.base_data().task_name, task_data.into());
    }

    pub fn insert_tasks<T: TaskData>(&mut self, tasks: Vec<T>) {
        for task in tasks {
            self.insert_task(task);
        }
    }

    /// # Params
    /// - controller: (controller: Box<dyn Controller>, controller_config: ResourceData) controller and its config resource data
    pub fn insert_controller(&mut self, controller: (Box<dyn Controller>, ResourceData)) {
        self.controllers.insert(
            controller.0.name(),
            ControllerWrapper::new(controller.0, controller.1),
        );
    }

    /// # Prams
    /// - controllers: Vec<Box<dyn Controller>> controllers and their config resource data
    pub fn inert_controllers(&mut self, controllers: Vec<(Box<dyn Controller>, ResourceData)>) {
        for controller in controllers {
            self.insert_controller(controller);
        }
    }

    pub fn insert_recognizer(&mut self, recognizer: Box<dyn Recognizer>) {
        self.reconizers.insert(recognizer.name(), recognizer.into());
    }

    pub fn insert_recognizers(&mut self, recognizers: Vec<Box<dyn Recognizer>>) {
        for recognizer in recognizers {
            self.insert_recognizer(recognizer);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
