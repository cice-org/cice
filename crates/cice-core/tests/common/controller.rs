use cice_core::{
    config::BaseControllerConfig,
    controller::{
        input::InputController,
        output::{image::ImageOutputController, OutputController},
        Controller,
    },
    resource::ResourceData,
};

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
        return "text_controller".into();
    }

    fn init(
        &self,
        resource: &cice_core::resource::ResourceData,
    ) -> Result<(), Box<dyn core::error::Error>> {
        let config: TestControllerConfig = serde_json::from_value(resource.clone())?;
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

impl InputController for TestController {
    fn exec(&self, input_action: &ResourceData) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl ImageOutputController for TestController {
    fn exec(
        &self,
        output_action: &ResourceData,
    ) -> Result<cice_core::controller::output::image::ImageOutput, Box<dyn std::error::Error>> {
        todo!()
    }
}

impl OutputController for TestController {}
