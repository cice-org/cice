pub mod action;
pub mod controller;
pub mod recognizer;
pub mod task;
pub mod resourece;

use controller::TestControllerConfig;
use recognizer::TestImageRecognizerConfig;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestConfig {
    pub controller: Option<TestControllerConfig>,
    pub recognizer: Option<TestImageRecognizerConfig>,
}
