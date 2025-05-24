use std::str::FromStr;

use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer, RecognizerError},
    resource::ResourceData,
};
use serde_json::json;
use snafu::Snafu;

pub struct AcceptAllRecognizer {}

impl Recognizer for AcceptAllRecognizer {
    fn name(&self) -> String {
        "recognizer_AcceptAll".into()
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
impl ImageRecognizer for AcceptAllRecognizer {
    async fn exec(
        &self,
        _action: &ResourceData,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(serde_json::Value::from_str(RECO_ACCEPT_ALL_RESULT).unwrap());
    }
}

pub const RECO_ACCEPT_ALL_RESULT: &str = r#"{"value":"Accept_All"}"#;
