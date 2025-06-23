pub mod action;
pub mod controller;
pub mod recognizer;
pub mod resource;
pub mod task;

use controller::ControllerConfig;
use recognizer::RecognizerConfig;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub controller: Option<ControllerConfig>,
    pub recognizer: Option<RecognizerConfig>,
}
