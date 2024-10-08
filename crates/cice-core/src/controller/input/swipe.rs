use alloc::boxed::Box;
use core::{error::Error, time::Duration};
use euclid::Point2D;

use super::{InputController, Px};

pub trait SwipeController: InputController {
    fn swipe(
        &self,
        from: Point2D<usize, Px>,
        to: Point2D<usize, Px>,
        duration: Option<Duration>,
    ) -> Result<(), Box<dyn Error>>;
}
