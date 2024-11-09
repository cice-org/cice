use crate::controller::Controller;
use crate::pipeline::Pipeline;
use crate::task::{Task, TaskData, TaskError, TaskId};
use std::collections::HashMap;

pub struct Context {
    pipeline: Pipeline,
    tasks: HashMap<TaskId, Task>,
}

impl Context {
    pub fn new() -> Context {
        Self {
            pipeline: Pipeline::new(),
            tasks: HashMap::new(),
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
    pub fn insert_task(&mut self, task_data: impl TaskData) {
        self.tasks
            .insert(task_data.base_data().task_name, task_data.into());
    }
    pub fn insert_controller<C: Controller>(&self, controller: C) {}
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
