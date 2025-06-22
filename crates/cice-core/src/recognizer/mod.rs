use core::error::Error;

use alloc::boxed::Box;
use async_trait::async_trait;
use snafu::Snafu;

use crate::{controller::ControllerData, resource::ResourceData};

pub type RecognizerId = String;
pub type RecognizeResult = ResourceData;

#[async_trait]
pub trait Recognizer: Send + Sync {
    fn name(&self) -> RecognizerId;
    /// ## Calling time
    /// Init Recognizer, would be called at the first time when the recognizer is called or any override config [recognizer_config_ext](crate::task::TaskData::recognizer_config_ext())
    /// ## Notice
    /// This is only necessary to be implemented when Recognizer supports `reinitialze` or needs `lazy initialize`. For most of the cases,
    /// keeping this function as a dummy implementation (by returning `Ok(())` directly) and passing an initialized and immutable structure (use `::new()` for example) is always the best solution
    fn init(&self, resource: &ResourceData) -> Result<(), RecognizerError>;
    fn require_input(&self) -> Option<ResourceData>; //Require input from a OutputController
    async fn exec(
        &self,
        action: Option<&ResourceData>,
        data: ControllerData,
    ) -> Result<RecognizeResult, CustomRecognizerError>;
}

#[derive(Debug, Snafu)]
pub enum RecognizerError {
    #[snafu(display("recognizer {id} not found"))]
    RecognizerNotFound { id: RecognizerId },
    /// ## Description
    /// Showing that the incoming ControllerData is not supported in this recognizer
    #[snafu(display("recognizer {id} is not compatible"))]
    IncompatibleRecognizer { id: RecognizerId },
    #[snafu(display("outer recognizer error {source}"))]
    Err { source: CustomRecognizerError }, //Customized Error passed by users
}

#[derive(Debug, Snafu)]
pub enum CustomRecognizerError {
    #[snafu(display("invalid action: {action:?}"))]
    InvalidAction { action: Option<ResourceData> },
    #[snafu(display("invalid data"))]
    InvalidData,
    #[snafu(display("fatal recognizer error: {source}"))]
    Fatal {
        source: Box<dyn Error + Send + Sync>,
    }, // Would cancel the whole program once emiited
    #[snafu(display("common recognizer error: {source}"))]
    Common {
        source: Box<dyn Error + Send + Sync>,
    },
}

impl From<CustomRecognizerError> for RecognizerError {
    fn from(value: CustomRecognizerError) -> Self {
        Self::Err { source: value }
    }
}
