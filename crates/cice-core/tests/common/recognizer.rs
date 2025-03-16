use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer, RecognizerError},
    resource::ResourceData,
};
use serde_json::json;
use snafu::Snafu;

use super::controller::BaseControllerConfig;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseRecognizerConfig {
    pub id: String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestImageRecognizerConfig {
    pub base: ::core::option::Option<BaseControllerConfig>,

    pub path: String,
}

pub struct TestImageRecognizer {}

impl Recognizer for TestImageRecognizer {
    fn name(&self) -> String {
        "recognizer_Image".into()
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
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({"name":"screen_capture"}))
    }
}

#[async_trait]
impl ImageRecognizer for TestImageRecognizer {
    async fn exec(
        &self,
        _action: &ResourceData,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
}

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
        return Ok(json!({}));
    }
}

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
        _action: &ResourceData,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Err(CustomRecognizerError::Common {
            source: Box::new(TestDenyAllError::DenyAll),
        });
    }
}

#[derive(Debug, Snafu)]
enum TestDenyAllError {
    #[snafu(display("deny all"))]
    DenyAll,
}
