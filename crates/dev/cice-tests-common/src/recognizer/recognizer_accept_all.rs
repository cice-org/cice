use std::str::FromStr;

use async_trait::async_trait;

use cice_core::controller::ControllerData;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{recognizer::Recognizer, resource::ResourceData};
use serde_json::json;

pub struct AcceptAllRecognizer {}

#[async_trait]
impl Recognizer for AcceptAllRecognizer {
    fn name(&self) -> String {
        "recognizer_AcceptAll".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(serde_json::Value::from_str(RECO_ACCEPT_ALL_RESULT).unwrap());
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}

pub const RECO_ACCEPT_ALL_RESULT: &str = r#"{"value":"Accept_All"}"#;
