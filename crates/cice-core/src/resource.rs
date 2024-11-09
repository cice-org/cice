use core::error::Error;

use alloc::{boxed::Box, string::String, vec::Vec};

pub enum ResourceData {
    Json(String),
    Proto(Vec<u8>),
}

pub enum ResourceError {
    Err(Box<dyn Error>), //Internal error thrown by caller
    InvalidResourceData,
}

impl ResourceData {
    // fn get_field(&self) -> T {
    //     match self {
    //         ResourceData::Json(s) => serde_json,
    //         ResourceData::Proto(vec) => prost::,
    //     }
    // }
}
