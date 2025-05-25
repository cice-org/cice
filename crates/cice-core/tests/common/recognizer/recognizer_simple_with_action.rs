use std::str::FromStr;

use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

pub struct SimpleRecognizerWithAction {}

impl Recognizer for SimpleRecognizerWithAction {
    fn name(&self) -> String {
        "recognizer_simple_with_action".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}

#[async_trait]
impl ImageRecognizer for SimpleRecognizerWithAction {
    async fn exec(
        &self,
        action: Option<&ResourceData>,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        assert_eq!(
            *action.unwrap().to_string(),
            serde_json::to_value(SIMPLE_RECOGNIZER_ACTION).unwrap()
        );
        return Ok(json!({}));
    }
}
const SIMPLE_RECOGNIZER_ACTION: &str = r#"{"area":[0,0,100,100]}"#;
