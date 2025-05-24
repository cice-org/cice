mod controller_simple_with_config;

pub use controller_simple_with_config::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum ControllerConfig {
    Simple(SimpleControllerConfig),
}
