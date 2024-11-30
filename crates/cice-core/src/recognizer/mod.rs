use core::error::Error;

use alloc::boxed::Box;
use result::ReconizeResult;

use crate::{action::reconizer::RecognizerAction, resource::ResourceData};

pub mod result;

pub type RecognizerId = String;

pub trait Recognizer {
    fn name(&self) -> RecognizerId;
    fn exec(
        &self,
        action: &dyn RecognizerAction,
        resource: &ResourceData,
    ) -> Result<Box<dyn ReconizeResult>, Box<dyn Error>>;
}
