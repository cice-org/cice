use crate::controller::Controller;
use core::time::Duration;
use euclid::Point2D;

use crate::define_controller;

use super::{InputController, Px};

pub trait ClickController: InputController {
    fn click(
        &mut self,
        pos: Point2D<usize, Px>,
        duration: Option<Duration>,
    ) -> Result<(), Self::Error>;
}


