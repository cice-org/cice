use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    config::BaseControllerConfig,
    recognizer::{image::ImageRecognizer, Recognizer, RecognizerError},
    resource::ResourceData,
};
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestImageRecognizerConfig {
    pub base: ::core::option::Option<BaseControllerConfig>,

    pub path: String,
}

pub struct TestImageRecognizer {}

impl Recognizer for TestImageRecognizer {
    fn name(&self) -> String {
        return "test_recognizer".into();
    }

    fn init(&self, resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        let config: TestImageRecognizerConfig =
            serde_json::from_value(resource.clone()).map_err(|e| RecognizerError::Err {
                source: CustomRecognizerError::Common {
                    source: Box::new(e),
                },
            })?;
        println!("path {}", config.path);
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        return Some(self);
    }

    fn require_input(&self) -> Option<ResourceData> {
        return Some(json!({"name":"screen_capture"}));
    }
}

#[async_trait]
impl ImageRecognizer for TestImageRecognizer {
    async fn exec(
        &self,
        action: &ResourceData,
        data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
}
