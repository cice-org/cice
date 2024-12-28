use cice_core::{config::BaseControllerConfig, recognizer::Recognizer, resource::ResourceData};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct TestRecognizerConfig {
    #[prost(message, optional, tag = "1")]
    pub base: ::core::option::Option<BaseControllerConfig>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}

impl Recognizer for TestRecognizerConfig {
    fn name(&self) -> String {
        return "test_recognizer".into();
    }

    fn init(&self, resource: &ResourceData) -> Result<(), Box<dyn std::error::Error>> {
        match resource {
            ResourceData::Json(json) => {
                let config: TestRecognizerConfig = serde_json::from_str(json)?;
                println!("path {}", config.path);
                Ok(())
            }
            ResourceData::Proto(data) => {
                let config: TestRecognizerConfig = prost::Message::decode(data.as_slice())?;
                println!("path {}", config.path);
                Ok(())
            }
        }
    }

    fn exec(
        &self,
        action: &dyn cice_core::action::reconizer::RecognizerAction,
    ) -> Result<Box<dyn cice_core::recognizer::result::ReconizeResult>, Box<dyn std::error::Error>>
    {
        todo!()
    }
}
