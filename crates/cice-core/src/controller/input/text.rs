use core::error::Error;

use super::InputController;
use alloc::boxed::Box;
use alloc::string::String;

pub trait TextController: InputController {
    fn input_text(&mut self, text: String) -> Result<(), Box<dyn Error>>;
}
