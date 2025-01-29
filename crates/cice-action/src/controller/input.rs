use euclid::{Box2D, Point2D};
use serde::{Deserialize, Serialize};

use crate::Action;

pub type Px = ();
pub type Point = Point2D<u64, Px>;
pub type Rect = Box2D<u64, Px>;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControllerInputAction {
    Click(ClickPostion),
}

impl Action for ControllerInputAction {}

#[derive(Serialize, Deserialize)]
pub enum ClickPostion {
    Point(Point),
    Rect(Rect),
}
