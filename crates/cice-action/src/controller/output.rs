use serde::{Deserialize, Serialize};

use crate::types::Rect;

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum ControllerOutputAction {
    #[serde(rename = "screen_capture")]
    ScreenCapture(ScreenCaptureTarget),
}

pub type WindowId = u64;
#[derive(Serialize, Deserialize)]
pub enum ScreenCaptureTarget {
    #[serde(rename = "window")]
    Window(ScreenCaptureWindowData),
}

#[derive(Serialize, Deserialize)]
pub struct ScreenCaptureWindowData {
    pub id: Option<WindowId>,
    pub area: Option<Rect>,
}
