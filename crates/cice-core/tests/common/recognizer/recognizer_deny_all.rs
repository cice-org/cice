use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer},
    resource::ResourceData,
};
use serde_json::json;
use snafu::Snafu;
pub struct DenyAllRecognizer {}

impl Recognizer for DenyAllRecognizer {
    fn name(&self) -> String {
        "recognizer_DenyAll".into()
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
impl ImageRecognizer for DenyAllRecognizer {
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Err(CustomRecognizerError::Common {
            source: Box::new(DenyAllError::DenyAll),
        });
    }
}

#[derive(Debug, Snafu)]
enum DenyAllError {
    #[snafu(display("deny all"))]
    DenyAll,
}
