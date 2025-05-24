use std::{fs::File, str::FromStr};

use async_trait::async_trait;
use cice_core::{
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller, ControllerError, CustomControllerError,
    },
    resource::ResourceData,
};

use crate::{Image, ImageData};

pub struct SimpleImageController {}

impl SimpleImageController {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Controller for SimpleImageController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "controller_simple_image".into()
    }

    fn init(&self, resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
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
impl InputController for SimpleImageController {
    async fn exec(&self, _input_action: &ResourceData) -> Result<(), CustomControllerError> {
        Ok(())
    }
}

#[async_trait]
impl ImageOutputController for SimpleImageController {
    async fn exec(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image);
    }
}

impl OutputController for SimpleImageController {
    fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
        Some(self)
    }
}
