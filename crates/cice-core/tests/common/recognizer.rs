use cice_core::{
    config::BaseControllerConfig,
    recognizer::{image::ImageRecognizer, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct TestImageRecognizerConfig {
    pub base: ::core::option::Option<BaseControllerConfig>,

    pub path: String,
}

pub struct TestImageRecognizer {}

impl Recognizer for TestImageRecognizer {
    fn name(&self) -> String {
        return "test_recognizer".into();
    }

    fn init(&self, resource: &ResourceData) -> Result<(), Box<dyn core::error::Error>> {
        let config: TestImageRecognizerConfig = serde_json::from_value(resource.clone())?;
        println!("path {}", config.path);
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        return Some(self);
    }

    fn require_input(&self) -> Option<ResourceData> {
        return Some(json!({"name":"screen_capture"}));
    }
}

impl ImageRecognizer for TestImageRecognizer {
    fn exec(
        &self,
        action: &ResourceData,
    ) -> Result<cice_core::recognizer::RecognizeResult, Box<dyn std::error::Error>> {
        todo!()
    }
}
