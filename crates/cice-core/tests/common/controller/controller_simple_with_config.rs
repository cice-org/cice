use std::str::FromStr;

use async_trait::async_trait;
use cice_core::{
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller, ControllerError, CustomControllerError,
    },
    resource::ResourceData,
};

use crate::{
    common::recognizer::{SimpleRecognizerConfig, RECO_ACCEPT_ALL_RESULT},
    Image,
};

use super::ControllerConfig;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseControllerConfig {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SimpleControllerConfig {
    pub base: Option<BaseControllerConfig>,
    pub port: String,
}

impl TryFrom<ControllerConfig> for SimpleControllerConfig {
    type Error = ();

    fn try_from(value: ControllerConfig) -> Result<Self, Self::Error> {
        match value {
            ControllerConfig::Simple(simple_controller_config) => {
                return Ok(simple_controller_config)
            }
            _ => return Err(()),
        }
    }
}

pub struct SimpleControllerWithConfig {
    config: SimpleControllerConfig,
}

impl SimpleControllerWithConfig {
    pub fn new(config: SimpleControllerConfig) -> Self {
        return Self { config };
    }
}

impl Controller for SimpleControllerWithConfig {
    fn name(&self) -> cice_core::controller::ControllerId {
        "controller_simple_with_config".into()
    }

    fn init(&self, resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
        assert_eq!(
            *resource,
            serde_json::to_value(self.config.clone()).unwrap()
        );
        Ok(())
    }
    fn ext_input(&self) -> Option<cice_core::controller::InputControllerOps> {
        Some(self)
    }
    fn ext_output(&self) -> Option<cice_core::controller::OutputControllerOps> {
        Some(self)
    }
}

#[async_trait]
impl InputController for SimpleControllerWithConfig {
    async fn exec(&self, _input_action: &ResourceData) -> Result<(), CustomControllerError> {
        assert_eq!(
            *_input_action,
            serde_json::Value::from_str(RECO_ACCEPT_ALL_RESULT).unwrap()
        );
        Ok(())
    }
}

#[async_trait]
impl ImageOutputController for SimpleControllerWithConfig {
    async fn exec(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image);
    }
}

impl OutputController for SimpleControllerWithConfig {
    fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
        Some(self)
    }
}
