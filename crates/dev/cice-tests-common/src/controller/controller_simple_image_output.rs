use async_trait::async_trait;
use cice_core::{
    controller::{Controller, ControllerError, CustomControllerError},
    resource::ResourceData,
};

use crate::Image;

pub struct SimpleImageController {}

impl Default for SimpleImageController {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleImageController {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Controller for SimpleImageController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "controller_simple_image".into()
    }

    fn init(&self, _resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
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
