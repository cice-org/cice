use core::error::Error;

use alloc::boxed::Box;
use image::ImageRecognizer;

use crate::resource::ResourceData;

pub mod image;

pub type RecognizerId = String;
pub type RecognizeResult = ResourceData;

pub trait Recognizer {
    fn name(&self) -> RecognizerId;
    fn init(&self, resource: &ResourceData) -> Result<(), Box<dyn Error>>;
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
