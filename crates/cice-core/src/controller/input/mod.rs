use super::Controller;
use crate::action::controller::input::ControllerInputAction;
use core::error::Error;
  
pub trait InputController: Controller {
    fn exec(&self, output_action: ControllerInputAction) -> Result<(), Box<dyn Error>>;
}
