use alloc::boxed::Box;
use core::{error::Error, time::Duration};
use euclid::Point2D;

use super::{InputController, Px};

pub trait ClickController: InputController {
    fn click(
        &mut self,
        pos: Point2D<usize, Px>,
        duration: Option<Duration>,
    ) -> Result<(), Box<dyn Error>>;
}
