use alloc::{collections::vec_deque::VecDeque, vec};

use crate::{context::Context, task::Task};

pub(crate) struct Pipeline {
    task_list: VecDeque<Task>,
}

impl Pipeline {
    pub(crate) fn new() -> Self {
        Self {
            task_list: vec![].into(),
        }
    }
    pub(crate) fn run_pipeline(&mut self, entry: Task, context: &Context) {}
}
