use alloc::boxed::Box;

use crate::{controller::Controller, recognizer::Recognizer};

use super::state::State;

pub struct Task {
    pub(crate) from: State,
    pub(crate) to: State,
    recognizer: Box<dyn Recognizer>,
    controller: Box<dyn Controller>,
}
