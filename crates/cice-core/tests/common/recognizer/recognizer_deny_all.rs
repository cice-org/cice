use async_trait::async_trait;
use cice_core::controller::ControllerData;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{recognizer::Recognizer, resource::ResourceData};
use serde_json::json;
use snafu::Snafu;
pub struct DenyAllRecognizer {}

#[async_trait]
impl Recognizer for DenyAllRecognizer {
    fn name(&self) -> String {
        "recognizer_DenyAll".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Err(CustomRecognizerError::Common {
            source: Box::new(DenyAllError::DenyAll),
        });
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}

#[derive(Debug, Snafu)]
enum DenyAllError {
    #[snafu(display("deny all"))]
    DenyAll,
}
