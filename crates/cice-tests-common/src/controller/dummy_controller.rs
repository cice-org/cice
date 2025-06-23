use async_trait::async_trait;
use cice_core::{
    controller::{Controller, ControllerError, CustomControllerError, ImageData},
    resource::ResourceData,
};

use crate::Image;

pub struct DummyController {}

impl DummyController {
    pub fn new() -> Self {
        return Self {};
    }
}

#[async_trait]
impl Controller for DummyController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "dummy_controller".into()
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
