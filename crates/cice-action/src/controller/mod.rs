pub mod input;
pub mod output;

use input::ControllerInputAction;
use output::ControllerOutputAction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControllerAction {
    Input(ControllerInputAction),
    Output(ControllerOutputAction),
}
#[cfg(test)]
mod test {
    use crate::controller::output::ScreenCaptureWindowData;

    use super::{
        output::{ControllerOutputAction, ScreenCaptureTarget},
        ControllerAction,
    };

    #[test]
    fn test_flatten_enum() {
        let screen_cap_action = ControllerAction::Output(ControllerOutputAction::ScreenCapture(
            ScreenCaptureTarget::Window(ScreenCaptureWindowData {
                id: None,
                area: None,
            }),
        ));

        assert_eq!(
            r#"{"screen_capture":{"window":{"id":null,"area":null}}}"#,
            serde_json::to_string(&screen_cap_action).unwrap()
        )
    }
}
