use cice_core::{config::BaseControllerConfig, controller::Controller, resource::ResourceData};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TestControllerConfig {
    #[prost(message, optional, tag = "1")]
    pub base: Option<BaseControllerConfig>,
    #[prost(string, tag = "2")]
    pub port: ::prost::alloc::string::String,
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        match resource {
            cice_core::resource::ResourceData::Json(json) => {
                let config: TestControllerConfig = serde_json::from_str(json)?;
                println!("port: {}", config.port);
                Ok(())
            }
            ResourceData::Proto(data) => {
                let config: TestControllerConfig = prost::Message::decode(data.as_slice())?;
                println!("port: {}", config.port);
                Ok(())
            }
        }
    }
}
