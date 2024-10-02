/// Input events or data to the target device. Such as click at a position, or input text to the device
pub mod input;
/// Fetch data from target device. Such as screen capture
pub mod output;
use alloc::string::String;
use core::error::Error;
use input::InputController;
use output::OutputController;

pub trait Controller {
    type Error: Error;
    fn name(&self) -> String;
    fn ext_input(&self) -> Option<InputControllerOps<Self>> {
        None
    }
    fn ext_output(&self) -> Option<OutputControllerOps<Self>> {
        None
    }
}

#[macro_export]
macro_rules! define_controller {
    ($exttrait:ident , $extname:ident) => {
        #[allow(missing_docs)]
        pub type $extname<'a, T> = &'a mut dyn $exttrait<Error = <T as Controller>::Error>;
    };
}

define_controller!(InputController, InputControllerOps);
define_controller!(OutputController, OutputControllerOps);
