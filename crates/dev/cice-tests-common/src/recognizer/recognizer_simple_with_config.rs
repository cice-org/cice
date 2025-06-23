use async_trait::async_trait;
use cice_core::{
    controller::ControllerData,
    recognizer::{CustomRecognizerError, Recognizer},
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
            RecognizerConfig::Simple(simple_recognizer_config) => Ok(simple_recognizer_config),
            _ => Err(()),
        }
    }
}

impl SimpleRecognizerWithConfig {
    pub fn new(config: SimpleRecognizerConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
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
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        _data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        return Ok(json!({}));
    }
    fn require_input(&self) -> Option<ResourceData> {
        None
    }
}
