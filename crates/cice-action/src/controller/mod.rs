pub mod input;
pub mod output;

use input::ControllerInputAction;
use output::ControllerOutputAction;
use serde::{Deserialize, Serialize};

use super::Action;
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ControllerAction {
    Input(ControllerInputAction),
    Output(ControllerOutputAction),
}

//TODO use a macro to define action
impl Action for ControllerAction {}

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
            ScreenCaptureTarget::Window(ScreenCaptureWindowData { id: None }),
        ));

        assert_eq!(
            r#"{"window":{"id":null}}"#,
            serde_json::to_string(&screen_cap_action).unwrap()
        )
    }
}
