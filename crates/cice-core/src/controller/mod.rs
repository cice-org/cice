/// Input events or data to the target device. Such as click at a position, or input text to the device
pub mod input;
/// Fetch data from target device. Such as screen capture
pub mod output;
use core::error::Error;

use alloc::string::String;
use input::InputController;
use output::OutputController;

use crate::resource::ResourceData;

pub type ControllerId = String;

pub trait Controller {
    fn name(&self) -> ControllerId;
    fn exec(&self, resourece: &ResourceData) -> Result<(), Box<dyn Error>>;
    fn ext_input(&self) -> Option<InputControllerOps> {
        None
    }
    fn ext_output(&self) -> Option<OutputControllerOps> {
        None
    }
}

#[macro_export]
macro_rules! define_controller {
    ($exttrait:ident , $extname:ident) => {
        #[allow(missing_docs)]
        pub type $extname<'a> = &'a mut dyn $exttrait;
    };
}

define_controller!(InputController, InputControllerOps);
define_controller!(OutputController, OutputControllerOps);
