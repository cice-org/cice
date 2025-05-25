use async_trait::async_trait;
use cice_core::{
    controller::output::image::ImageOutput,
    recognizer::{image::ImageRecognizer, CustomRecognizerError, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

use super::{BaseRecognizerConfig, RecognizerConfig};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SimpleRecognizerConfig {
    pub base: ::core::option::Option<BaseRecognizerConfig>,
    pub path: String,
}
pub struct SimpleRecognizerWithConfig {
    config: SimpleRecognizerConfig,
}

impl TryFrom<RecognizerConfig> for SimpleRecognizerConfig {
    type Error = ();

    fn try_from(value: RecognizerConfig) -> Result<Self, Self::Error> {
        match value {
            RecognizerConfig::Simple(simple_recognizer_config) => {
                return Ok(simple_recognizer_config)
            }
            _ => return Err(()),
        }
    }
}

impl SimpleRecognizerWithConfig {
    pub fn new(config: SimpleRecognizerConfig) -> Self {
        return Self { config };
    }
}

impl Recognizer for SimpleRecognizerWithConfig {
    fn name(&self) -> String {
        "recognizer_simple_with_config".into()
    }

    fn init(&self, resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        assert_eq!(
            *resource,
            serde_json::to_value(self.config.clone()).unwrap()
        );
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        None
    }
}

#[async_trait]
impl ImageRecognizer for SimpleRecognizerWithConfig {
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
}
