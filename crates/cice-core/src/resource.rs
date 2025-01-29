use core::error::Error;

use alloc::{boxed::Box, string::String, vec::Vec};
use serde_json::Value;

pub type ResourceData = Value;
// //TODO: make it easier to be used. There is no need for the caller to care about how to deserialize the data actually.
// #[non_exhaustive]
// pub enum ResourceData {
//     Json(Value),
//     Proto(Vec<u8>),
// }

// pub enum ResourceError {
//     Err(Box<dyn Error>), //Internal error thrown by caller
//     InvalidResourceData,
// }
