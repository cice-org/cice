pub mod input;
pub mod output;

use super::Action;
pub trait ControllerAction: Action {}
