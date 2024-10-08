pub mod controller;
pub mod reconizer;

use alloc::string::String;

pub trait Action {
    fn name(&self) -> String;
}
