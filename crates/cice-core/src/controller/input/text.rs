use super::InputController;
use crate::controller::Controller;
use crate::define_controller;
use alloc::string::String;

pub trait TextController: InputController {
    fn input_text(&mut self, text: String) -> Result<(), Self::Error>;
}


