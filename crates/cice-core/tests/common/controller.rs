use async_trait::async_trait;
use cice_core::{
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller, ControllerError, CustomControllerError,
    },
    resource::ResourceData,
};

use crate::TestImage;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseControllerConfig {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestControllerConfig {
    pub base: Option<BaseControllerConfig>,
    pub port: String,
}

pub struct TestController {}

// #[derive(Serialize, Deserialize, Message)]
// pub struct TestControllerConfig {
//     #[prost(string, tag = "1")]
//     port: String,
// }

impl Controller for TestController {
    fn name(&self) -> cice_core::controller::ControllerId {
        "test_controller".into()
    }

    fn init(&self, resource: &cice_core::resource::ResourceData) -> Result<(), ControllerError> {
        let config: TestControllerConfig =
            serde_json::from_value(resource.clone()).map_err(|e| ControllerError::Err {
                source: CustomControllerError::Common {
                    source: Box::new(e),
                },
            })?;
        println!("port: {}", config.port);
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
impl InputController for TestController {
    async fn exec(&self, _input_action: &ResourceData) -> Result<(), CustomControllerError> {
        todo!()
    }
}

#[async_trait]
impl ImageOutputController for TestController {
    async fn exec(
        &self,
        _output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, CustomControllerError> {
        let image = image::open(TestImage!("testCase.jpg")).unwrap();
        return Ok(image);
    }
}

impl OutputController for TestController {
    fn ext_image(&self) -> Option<cice_core::controller::output::ImageOutputControllerOps> {
        Some(self)
    }
}
