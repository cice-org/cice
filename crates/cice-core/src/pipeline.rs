use alloc::{collections::vec_deque::VecDeque, vec};

use crate::{
    context::Context,
    task::{Task, TaskError},
};

pub(crate) struct Pipeline {
    task_list: VecDeque<Task>,
}

impl Pipeline {
    pub(crate) fn new() -> Self {
        Self {
            task_list: vec![].into(),
        }
    }
    pub(crate) async fn run_pipeline(
        &self,
        entry: Task,
        context: &Context,
    ) -> Result<(), TaskError> {
        Ok(())
    }
}
