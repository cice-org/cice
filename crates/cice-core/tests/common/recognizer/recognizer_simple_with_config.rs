use async_trait::async_trait;
use cice_core::{
    controller::output::image::ImageOutput,
    recognizer::{image::ImageRecognizer, CustomRecognizerError, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

use super::{BaseRecognizerConfig, TestRecognizerConfig};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestSimpleRecognizerConfig {
    pub base: ::core::option::Option<BaseRecognizerConfig>,
    pub path: String,
}
pub struct TestSimpleRecognizerWithConfig {
    config: TestSimpleRecognizerConfig,
}

impl TryFrom<TestRecognizerConfig> for TestSimpleRecognizerConfig {
    type Error = ();

    fn try_from(value: TestRecognizerConfig) -> Result<Self, Self::Error> {
        match value {
            TestRecognizerConfig::Simple(test_simple_recognizer_config) => {
                return Ok(test_simple_recognizer_config)
            }
            _ => return Err(()),
        }
    }
}

impl TestSimpleRecognizerWithConfig {
    pub fn new(config: TestSimpleRecognizerConfig) -> Self {
        return Self { config };
    }
}

impl Recognizer for TestSimpleRecognizerWithConfig {
    fn name(&self) -> String {
        "recognizer_simple_with_config".into()
    }

    fn init(&self, resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        assert_eq!(
            *resource,
            serde_json::to_value(self.config.clone()).unwrap()
        );
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        None
    }
}

#[async_trait]
impl ImageRecognizer for TestSimpleRecognizerWithConfig {
    async fn exec(
        &self,
        _action: &ResourceData,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
}
