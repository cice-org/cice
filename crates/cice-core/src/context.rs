use alloc::collections::vec_deque::VecDeque;

use crate::pipeline::Pipeline;
use crate::task::Task;

pub struct Context {
    task_list: VecDeque<Task>,
}

impl Context {
    pub fn new() -> Context {
        Self {
            task_list: VecDeque::new(),
        }
    }
    pub fn run(&mut self, entry: Task) {
        self.task_list.push_back(entry);
        while let Some(task) = self.task_list.pop_front() {
            task.run_with_context(self);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
