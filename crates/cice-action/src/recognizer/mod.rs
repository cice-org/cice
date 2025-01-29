pub mod image;

use cice_core::recognizer::RecognizeResult;
use image::ImageRecognizeAction;
use serde::{Deserialize, Serialize};

use super::Action;

#[derive(Serialize, Deserialize)]
pub enum RecognizerAction {
    Direct(RecognizeResult),
    Image(ImageRecognizeAction),
}

impl Action for RecognizerAction {}
