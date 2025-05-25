use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

pub struct AssertImageRecognizer {}

impl Recognizer for AssertImageRecognizer {
    fn name(&self) -> String {
        "recognizer_assert_image".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({"name":"screen_capture"}))
    }
}

#[async_trait]
impl ImageRecognizer for AssertImageRecognizer {
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
}
