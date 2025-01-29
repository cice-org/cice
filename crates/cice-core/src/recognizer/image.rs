use core::error::Error;

use crate::resource::ResourceData;

use super::{RecognizeResult, Recognizer};

pub trait ImageRecognizer: Recognizer {
    fn exec(&self, action: &ResourceData) -> Result<RecognizeResult, Box<dyn Error>>;
}
