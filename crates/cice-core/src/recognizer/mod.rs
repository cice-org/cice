use core::error::Error;

use alloc::boxed::Box;
use image::ImageRecognizer;
use snafu::Snafu;

use crate::resource::ResourceData;

pub mod image;

pub type RecognizerId = String;
pub type RecognizeResult = ResourceData;

pub trait Recognizer: Send + Sync {
    fn name(&self) -> RecognizerId;
    fn init(&self, resource: &ResourceData) -> Result<(), RecognizerError>;
    fn require_input(&self) -> Option<ResourceData>; //Require input from a OutputController
    fn ext_image(&self) -> Option<ImageRecognizerOps> {
        None
    }
}

#[macro_export]
macro_rules! define_recognizer {
    ($exttrait:ident , $extname:ident) => {
        #[allow(missing_docs)]
        pub type $extname<'a> = &'a dyn $exttrait;
    };
}

define_recognizer!(ImageRecognizer, ImageRecognizerOps);

#[derive(Debug, Snafu)]
pub enum RecognizerError {
    #[snafu(display("recognizer {id} not found"))]
    RecognizerNotFound { id: RecognizerId },
    #[snafu(display("recognizer {id} is not compatible"))]
    IncompatibleRecognizer { id: RecognizerId },
    #[snafu(display("outer recognizer error {source}"))]
    Err { source: CustomRecognizerError }, //Customized Error passed by users
}

#[derive(Debug, Snafu)]
pub enum CustomRecognizerError {
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
