#![no_std]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;
pub mod controller;
pub mod executor;
pub mod graph;
pub mod recognizer;
