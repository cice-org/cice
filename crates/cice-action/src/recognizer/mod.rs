pub mod image;

use cice_core::recognizer::RecognizeResult;
use image::ImageRecognizeAction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RecognizerAction {
    Direct(RecognizeResult),
    Image(ImageRecognizeAction),
}
