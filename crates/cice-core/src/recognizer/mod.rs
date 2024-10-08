use core::error::Error;

use alloc::boxed::Box;
use result::ReconizeResult;

use crate::action::reconizer::RecognizerAction;

pub mod result;

pub trait Recognizer {
    type Action: RecognizerAction;
    fn exec(&self, action: &Self::Action) -> Result<Box<dyn ReconizeResult>, Box<dyn Error>>;
}
struct A;
impl Iterator for A{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl A {
    fn a(&self){
        self.into_iter()
    }
}