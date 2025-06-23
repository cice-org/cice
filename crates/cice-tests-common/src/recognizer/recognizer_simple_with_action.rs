use async_trait::async_trait;

use cice_core::controller::ControllerData;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{recognizer::Recognizer, resource::ResourceData};
use serde_json::json;

pub struct SimpleRecognizerWithAction {}

#[async_trait]
impl Recognizer for SimpleRecognizerWithAction {
    fn name(&self) -> String {
        "recognizer_simple_with_action".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    async fn exec(
        &self,
        action: Option<&ResourceData>,
        _data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        assert_eq!(
            *action.unwrap().to_string(),
            serde_json::to_value(SIMPLE_RECOGNIZER_ACTION).unwrap()
        );
        return Ok(json!({}));
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}

const SIMPLE_RECOGNIZER_ACTION: &str = r#"{"area":[0,0,100,100]}"#;
