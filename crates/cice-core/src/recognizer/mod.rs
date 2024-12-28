use core::error::Error;

use alloc::boxed::Box;
use result::ReconizeResult;

use crate::{action::reconizer::RecognizerAction, resource::ResourceData};

pub mod result;

pub type RecognizerId = String;

pub trait Recognizer {
    fn name(&self) -> RecognizerId;
    fn init(&self, resource: &ResourceData) -> Result<(), Box<dyn Error>>;
    fn exec(
        &self,
        action: &dyn RecognizerAction,
    ) -> Result<Box<dyn ReconizeResult>, Box<dyn Error>>;
}
