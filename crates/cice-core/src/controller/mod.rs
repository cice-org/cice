/// Input events or data to the target device. Such as click at a position, or input text to the device
pub mod input;
/// Fetch data from target device. Such as screen capture
pub mod output;
use core::error::Error;
use std::sync::Arc;

use alloc::string::String;
use input::InputController;
use output::OutputController;
use snafu::Snafu;

use crate::resource::ResourceData;

pub type ControllerId = String;

/// Controller is a trait that defines the behavior of a controller
///
/// ## Tips
/// - Controller is lazy initialized, which means the controller will be initialized when it is used for the first time
pub trait Controller: Send + Sync {
    fn name(&self) -> ControllerId;
    fn init(&self, init_cofig: &ResourceData) -> Result<(), ControllerError>;
    // Should any controller implement both input and output at once?
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
        pub type $extname<'a> = &'a dyn $exttrait;
    };
}

define_controller!(InputController, InputControllerOps);
define_controller!(OutputController, OutputControllerOps);

#[derive(Debug, Snafu)]
pub enum ControllerError {
    #[snafu(display("controller not found: {id}"))]
    ControllerNotFound { id: ControllerId },
    #[snafu(display("custom controller error: {source}"))]
    Err { source: CustomControllerError }, //Customized Error passed by users
}

#[derive(Debug, Snafu)]
pub enum CustomControllerError {
    #[snafu(display("fatal controller error: {source}"))]
    Fatal {
        source: Box<dyn Error + Send + Sync>,
    }, //Would cancel the whole program once emiited
    #[snafu(display("common controller error: {source}"))]
    Common {
        source: Box<dyn Error + Send + Sync>,
    },
}

impl From<CustomControllerError> for ControllerError {
    fn from(value: CustomControllerError) -> Self {
        Self::Err { source: value }
    }
}
