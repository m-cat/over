//! Error module.

use std::{error, fmt};

/// The fabulous OVER error type.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FieldNotFound(String),
    WrongTypeFound(String),
    UnknownError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            FieldNotFound(ref field) => write!(f, "Field not found: {}", field),
            WrongTypeFound(ref field) => write!(f, "Wrong type found for field: {}", field),
            UnknownError => write!(f, "An unknown error has occurred"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            FieldNotFound(_) => "Field not found: {}",
            WrongTypeFound(_) => "Wrong type found",
            UnknownError => "An unknown error has occurred",
        }
    }
}
