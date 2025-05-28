use std::str::FromStr;

use crate::common::recognizer::RECO_ACCEPT_ALL_RESULT;
use crate::Image;
use async_trait::async_trait;
use cice_core::controller::{ControllerData, ImageData};
use cice_core::recognizer::CustomRecognizerError;
use cice_core::{recognizer::Recognizer, resource::ResourceData};
use serde_json::json;

pub struct SimpleImageInputRecognizer {}

#[async_trait]
impl Recognizer for SimpleImageInputRecognizer {
    fn name(&self) -> String {
        "recognizer_simple_image_input".into()
    }

    fn init(&self, _resource: &ResourceData) -> Result<(), cice_core::recognizer::RecognizerError> {
        Ok(())
    }
    async fn exec(
        &self,
        _action: Option<&ResourceData>,
        data: ControllerData,
    ) -> Result<cice_core::recognizer::RecognizeResult, CustomRecognizerError> {
        let data: ImageData = data.try_into().unwrap();
        assert_eq!(
            data.as_bytes(),
            image::open(Image!("testCase.jpg")).unwrap().as_bytes()
        );
        return Ok(serde_json::Value::from_str(RECO_ACCEPT_ALL_RESULT).unwrap());
    }

    fn require_input(&self) -> Option<ResourceData> {
        Some(json!({}))
    }
}
