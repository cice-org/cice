use std::str::FromStr;

use crate::common::recognizer::RECO_ACCEPT_ALL_RESULT;
use crate::Image;
use async_trait::async_trait;
use cice_core::controller::output::image::ImageOutput;
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{
    recognizer::{image::ImageRecognizer, Recognizer},
    resource::ResourceData,
};
use serde_json::json;

pub struct SimpleImageInputRecognizer {}

impl Recognizer for SimpleImageInputRecognizer {
    fn name(&self) -> String {
        "recognizer_simple_image_input".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    fn ext_image(&self) -> Option<cice_core::recognizer::ImageRecognizerOps> {
        Some(self)
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}

#[async_trait]
impl ImageRecognizer for SimpleImageInputRecognizer {
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        data: ImageOutput,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        assert_eq!(
            data.as_bytes(),
            image::open(Image!("testCase.jpg")).unwrap().as_bytes()
        );
        return Ok(serde_json::Value::from_str(RECO_ACCEPT_ALL_RESULT).unwrap());
    }
}
