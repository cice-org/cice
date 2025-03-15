use async_trait::async_trait;
use image::DynamicImage;

use crate::{controller::CustomControllerError, resource::ResourceData};

use super::OutputController;

pub type ImageOutput = DynamicImage;

#[async_trait]
pub trait ImageOutputController: OutputController {
    async fn exec(&self, action: &ResourceData) -> Result<ImageOutput, CustomControllerError>;
}
