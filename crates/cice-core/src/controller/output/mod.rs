use super::Controller;
use crate::action::controller::output::ControllerOutputAction;
use image::ImageOutput;
use std::error::Error;

pub mod image;

pub enum OutputData {
    Image(ImageOutput),
}

pub trait OutputController: Controller {
    fn exec(&self, output_action: ControllerOutputAction) -> Result<OutputData, Box<dyn Error>>;
}
