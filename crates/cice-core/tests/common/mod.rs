pub mod action;
pub mod controller;
pub mod recognizer;
pub mod task;

use cice_core::{
    action::Action,
    task::{BaseTaskData, TaskData},
};
use controller::TestControllerConfig;
use recognizer::TestRecognizerConfig;
use serde::{Deserialize, Serialize};



#[derive(serde::Serialize, serde::Deserialize, ::prost::Message)]
pub struct TestConfig {
    #[prost(message, optional, tag = "1")]
    pub controller: Option<TestControllerConfig>,
    #[prost(message, optional, tag = "2")]
    pub recognizer: Option<TestRecognizerConfig>,
}
