use async_trait::async_trait;
use cice_core::{
    controller::{Controller, ControllerError, CustomControllerError},
    resource::ResourceData,
};

use crate::Image;

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
            ControllerConfig::Simple(simple_controller_config) => Ok(simple_controller_config),
            _ => Err(()),
        }
    }
}

pub struct SimpleControllerWithConfig {
    config: SimpleControllerConfig,
}

impl SimpleControllerWithConfig {
    pub fn new(config: SimpleControllerConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
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
    async fn exec_input(&self, _input_action: &ResourceData) -> Result<(), CustomControllerError> {
        Ok(())
    }
    async fn exec_output(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::ControllerData, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image.into());
    }
}
