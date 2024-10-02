use core::cell::RefCell;

use alloc::{rc::Rc, vec::Vec};

use crate::{context::Context, controller::Controller, recognizer::Recognizer};

#[repr(transparent)]
pub struct Task(Rc<RefCell<TaskInner>>);

pub struct TaskInner {
    next: Vec<Task>, //Following tasks to be executed
    interrupt: Vec<Task>, // If none of tasks in next array is matched
                     // recognizer: Rc<dyn Recognizer>,
                     // controller: Rc<dyn Controller>,
}
impl Task {
    pub fn run_with_context(&self, context: &mut Context) {}
}
