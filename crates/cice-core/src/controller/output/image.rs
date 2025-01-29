use core::error::Error;

use image::DynamicImage;

use crate::resource::ResourceData;

use super::OutputController;

pub type ImageOutput = DynamicImage;

pub trait ImageOutputController: OutputController {
    fn exec(&self, action: &ResourceData) -> Result<ImageOutput, Box<dyn Error>>;
}
