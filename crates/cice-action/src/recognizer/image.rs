use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ImageRecognizeAction {
    Classify(),
}
