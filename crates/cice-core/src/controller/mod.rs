use core::error::Error;

use alloc::string::String;
use async_trait::async_trait;
use image::DynamicImage;
use snafu::Snafu;

use crate::resource::ResourceData;

pub type ControllerId = String;

/// Controller is a trait that defines the behavior of a controller
///
/// ## Tips
/// - Controller is lazy initialized, which means the controller will be initialized when it is used for the first time
#[async_trait]
pub trait Controller: Send + Sync {
    fn name(&self) -> ControllerId;
    /// ## Calling time
    /// Init Controller, would be called at the first time when the controller is called or any override config [controller_config_ext](crate::task::TaskData::controller_config_ext())
    ///## Notice
    /// This is only necessary to be implemented when Controller supports `reinitialze` or needs `lazy initialize`. For most of the cases,
    /// keeping this function as a dummy implementation (by returning `Ok(())` directly) and passing an initialized and immutable structure (use `::new()` for example) is always the best solution
    fn init(&self, init_cofig: &ResourceData) -> Result<(), ControllerError>;
    /// Fetch data from target device. Such as screen capture
    async fn exec_output(
        &self,
        action: &ResourceData,
    ) -> Result<ControllerData, CustomControllerError>;
    /// Input events or data to the target device. Such as click at a position, or input text to the device
    async fn exec_input(&self, action: &ResourceData) -> Result<(), CustomControllerError>;
}

#[derive(Debug, Snafu)]
pub enum ControllerError {
    #[snafu(display("controller not found: {id}"))]
    ControllerNotFound { id: ControllerId },
    #[snafu(display("custom controller error: {source}"))]
    Err { source: CustomControllerError }, //Customized Error passed by users
}

#[derive(Debug, Snafu)]
pub enum CustomControllerError {
    #[snafu(display("invalid action: {action:?}"))]
    InvalidAction { action: Option<ResourceData> },
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

pub type ImageData = DynamicImage;

#[non_exhaustive]
pub enum ControllerData {
    Image(ImageData),
    //TODO audio or more
}

impl TryFrom<ControllerData> for ImageData {
    type Error = ();

    fn try_from(value: ControllerData) -> Result<Self, Self::Error> {
        // Fix warning for now
        // if let ControllerData::Image(data) = value {
        //     return Ok(data);
        // } else {
        //     return Err(());
        // }
        let ControllerData::Image(data) = value;
        Ok(data)
    }
}

impl From<ImageData> for ControllerData {
    fn from(value: ImageData) -> Self {
        Self::Image(value)
    }
}
