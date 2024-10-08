use super::Controller;
use crate::define_controller;
use click::ClickController;
use swipe::SwipeController;
use text::TextController;

pub mod click;
pub mod swipe;
pub mod text;
type Px = ();
pub trait InputController: Controller {
    fn ext_click(&mut self) -> Option<ClickControllerOps> {
        None
    }
    fn ext_swipe(&mut self) -> Option<SwipeControllerOps> {
        None
    }
    fn ext_text(&mut self) -> Option<TextControllerOps> {
        None
    }
}

define_controller!(ClickController, ClickControllerOps);
define_controller!(SwipeController, SwipeControllerOps);
define_controller!(TextController, TextControllerOps);
