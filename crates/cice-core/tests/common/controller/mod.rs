mod controller_input_and_output_action;
mod controller_simple_image_output;
mod controller_simple_with_config;
mod dummy_controller;

pub use controller_input_and_output_action::*;
pub use controller_simple_image_output::*;
pub use controller_simple_with_config::*;
pub use dummy_controller::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum ControllerConfig {
    Simple(SimpleControllerConfig),
}
