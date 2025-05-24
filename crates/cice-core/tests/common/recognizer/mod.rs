mod recognizer_accept_all;
mod recognizer_assert_image;
mod recognizer_deny_all;
mod recognizer_simple_image_input;
mod recognizer_simple_with_config;
pub use recognizer_accept_all::*;
pub use recognizer_assert_image::*;
pub use recognizer_deny_all::*;
pub use recognizer_simple_image_input::*;
pub use recognizer_simple_with_config::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BaseRecognizerConfig {
    pub id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(untagged)]
pub enum RecognizerConfig {
    Simple(SimpleRecognizerConfig),
}
