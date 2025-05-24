pub mod action;
pub mod controller;
pub mod recognizer;
pub mod resource;
pub mod task;

use controller::TestControllerConfig;
use recognizer::TestRecognizerConfig;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TestConfig {
    pub controller: Option<TestControllerConfig>,
    pub recognizer: Option<TestRecognizerConfig>,
}
