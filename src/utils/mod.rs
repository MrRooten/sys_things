use std::error::Error;

pub mod func;
pub mod error;
pub mod cmd;
pub mod shellcode;
#[derive(Debug,Default)]
pub struct STError {
    detail  : String,
    err     : Option<Box<dyn Error>>
}