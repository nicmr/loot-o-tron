use core::num::dec2flt::parse;
use std::fmt;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseCommandError {
    #[error("Wrong Parameter Count provided. Required: {required:?}, Actual: {actual:?}")]
    WrongParameterCount{
        required: u32,
        actual: u32
    },
}