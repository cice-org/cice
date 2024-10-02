use crate::{controller::Controller, define_controller};
use core::time::Duration;
use euclid::Point2D;

use super::{InputController, Px};

pub trait SwipeController: InputController {
    fn swipe(from: Point2D<usize, Px>, to: Point2D<usize, Px>, duration: Option<Duration>);
}

