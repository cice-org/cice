use async_trait::async_trait;
use cice_core::{
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller, ControllerError, CustomControllerError,
    },
    resource::ResourceData,
};

use crate::Image;

pub struct DummyController {}

impl DummyController {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Controller for DummyController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "dummy_controller".into()
    }

    fn init(&self, _resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
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
impl InputController for DummyController {
    async fn exec(&self, _input_action: &ResourceData) -> Result<(), CustomControllerError> {
        Ok(())
    }
}

#[async_trait]
impl ImageOutputController for DummyController {
    async fn exec(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image);
    }
}

impl OutputController for DummyController {
    fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
        Some(self)
    }
}
