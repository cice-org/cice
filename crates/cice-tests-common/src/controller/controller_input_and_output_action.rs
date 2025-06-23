use async_trait::async_trait;
use cice_core::{
    controller::{Controller, ControllerError, CustomControllerError},
    resource::ResourceData,
};
use serde_json::json;

use crate::Image;

pub struct ControllerWithInputAndOutputAction {}

impl Default for ControllerWithInputAndOutputAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ControllerWithInputAndOutputAction {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Controller for ControllerWithInputAndOutputAction {
    fn name(&self) -> cice_core::controller::ControllerId {
        "controller_with_input_and_output_action".into()
    }

    fn init(&self, _resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
        Ok(())
    }

    async fn exec_input(&self, input_action: &ResourceData) -> Result<(), CustomControllerError> {
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
    async fn exec_output(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::ControllerData, CustomControllerError> {
        let image = image::open(Image!("testCase.jpg")).unwrap();
        return Ok(image.into());
    }
}
