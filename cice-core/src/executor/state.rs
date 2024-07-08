use core::cell::RefCell;

use alloc::{rc::Rc, string::String};

#[derive(Clone)]
pub struct State {
    pub(crate) inner: Rc<RefCell<StateInner>>,
    name: String,
}

pub(crate) struct StateInner {
    pub(crate) id: Option<usize>,
}

pub(crate) struct StateCondition{
    
}