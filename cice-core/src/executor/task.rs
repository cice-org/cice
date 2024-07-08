use alloc::boxed::Box;

use crate::{controller::Controller, driver::Driver};

use super::state::State;

pub struct Task {
    pub(crate) from: State,
    pub(crate) to: State,
    driver: Box<dyn Driver>,
    controller: Box<dyn Controller>,
}
