use async_trait::async_trait;

use cice_core::controller::ControllerData;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{recognizer::Recognizer, resource::ResourceData};
use serde_json::json;

pub struct AssertImageRecognizer {}

#[async_trait]
impl Recognizer for AssertImageRecognizer {
    fn name(&self) -> String {
        "recognizer_assert_image".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({"name":"screen_capture"}))
    }
}