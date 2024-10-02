pub mod action;
pub mod input;

pub trait Recognizer {
    type Input;
}
