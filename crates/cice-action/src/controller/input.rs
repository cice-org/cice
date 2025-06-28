use serde::{Deserialize, Serialize};

use crate::types::{Point, Rect};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum ControllerInputAction {
    #[serde(rename = "click")]
    Click { pos: ClickPostion },
    #[serde(rename = "swipe")]
    Swipe { from: Point, to: Point },
    #[serde(rename = "keyevent")]
    KeyEvent { id: usize },
}

#[derive(Serialize, Deserialize)]
pub enum ClickPostion {
    Point(Point),
    Rect(Rect),
}

#[cfg(test)]
mod test {
    use crate::controller::input::{ControllerInputAction, Point};

    #[test]
    fn deserialze() {
        println!(
            "{:?}",
            serde_json::to_string(&ControllerInputAction::Swipe {
                from: Point::zero(),
                to: Point::new(200, 200)
            })
        )
    }
}
