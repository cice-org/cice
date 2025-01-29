use crate::resource::ResourceData;

use super::Controller;
use core::error::Error;

pub trait InputController: Controller {
    fn exec(&self, action: &ResourceData) -> Result<(), Box<dyn Error>>;
}
