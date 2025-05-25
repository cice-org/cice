use async_trait::async_trait;
use cice_core::{
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller, ControllerError, CustomControllerError,
    },
    resource::ResourceData,
};
use serde_json::json;

use crate::Image;

pub struct ControllerWithInputAndOutputAction {}

impl ControllerWithInputAndOutputAction {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Controller for ControllerWithInputAndOutputAction {
    fn name(&self) -> cice_core::controller::ControllerId {
        "controller_with_input_and_output_action".into()
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
impl InputController for ControllerWithInputAndOutputAction {
    async fn exec(&self, input_action: &ResourceData) -> Result<(), CustomControllerError> {
        assert_eq!(
            serde_json::to_string(input_action).unwrap(),
            json!({
                "name":"click",
                "value":"Accept_All"
            })
            .to_string()
        );
        Ok(())
    }
}

#[async_trait]
impl ImageOutputController for ControllerWithInputAndOutputAction {
    async fn exec(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image);
    }
}

impl OutputController for ControllerWithInputAndOutputAction {
    fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
        Some(self)
    }
}
