use super::Controller;
use crate::define_controller;
use core::error::Error;
use image::{ImageOutput, ImageOutputController};

pub mod image;

pub trait OutputController: Controller {
    fn ext_image(&self) -> Option<ImageOutputControllerOps> {
        None
    }
}
define_controller!(ImageOutputController, ImageOutputControllerOps);
