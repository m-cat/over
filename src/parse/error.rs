//! Module for parse errors.

use std::error::Error;
use std::fmt;

/// Parse error type.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnknownError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;

        match *self {
            UnknownError => write!(f, "An unknown error has occurred"),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;

        match *self {
            UnknownError => "An unknown error has occurred",
        }
    }
}
