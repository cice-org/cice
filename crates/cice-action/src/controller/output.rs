use serde::{Deserialize, Serialize};

use crate::Action;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControllerOutputAction {
    ScreenCapture(ScreenCaptureTarget),
}

impl Action for ControllerOutputAction {}

pub type WindowId = u64;
#[derive(Serialize, Deserialize)]
pub enum ScreenCaptureTarget {
    #[serde(rename = "window")]
    Window(ScreenCaptureWindowData),
}

#[derive(Serialize, Deserialize)]
pub struct ScreenCaptureWindowData {
    pub id: Option<WindowId>,
}
