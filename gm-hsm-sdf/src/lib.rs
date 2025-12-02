// TODO: Temporarily allow dead_code during early development phase. Remove this attribute once the code is stabilized and unused items are cleaned up.
#![allow(dead_code)]

mod engine;
mod error;

pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;
