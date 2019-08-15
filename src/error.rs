//! Error module.

#![allow(missing_docs)]

use crate::{parse::error::ParseError, types::Type};
use std::{error::Error, fmt, io};

/// The fabulous OVER error type.
#[derive(Debug, PartialEq, Eq)]
pub enum OverError {
    ArrOutOfBounds(usize),
    ArrTypeMismatch(Type, Type),
    FieldNotFound(String),
    InvalidFieldName(String),
    NoParentFound,
    ParseError(String),
    TupOutOfBounds(usize),
    TupTypeMismatch(Type, Type, usize),
    TypeMismatch(Type, Type),

    IoError(String),
}

impl fmt::Display for OverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OverError::*;

        match *self {
            ArrOutOfBounds(ref index) => write!(f, "Arr index {} out of bounds", index),
            ArrTypeMismatch(ref expected, ref found) => write!(
                f,
                "Arr inner types do not match: expected {}, found {}",
                expected, found
            ),
            FieldNotFound(ref field) => write!(f, "Field not found: \"{}\"", field),
            InvalidFieldName(ref field) => write!(f, "Invalid field name: \"{}\"", field),
            NoParentFound => write!(f, "No parent found for this obj"),
            TupOutOfBounds(ref index) => write!(f, "Tup index {} out of bounds", index),
            TupTypeMismatch(ref expected, ref found, ref index) => write!(
                f,
                "Tup inner types do not match at index {}: expected {}, found {}",
                index, expected, found
            ),
            TypeMismatch(ref expected, ref found) => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }

            ParseError(ref error) | IoError(ref error) => write!(f, "{}", error),
        }
    }
}

impl Error for OverError {}

impl From<io::Error> for OverError {
    fn from(e: io::Error) -> Self {
        OverError::IoError(format!("{}", e))
    }
}

impl From<ParseError> for OverError {
    fn from(e: ParseError) -> Self {
        OverError::ParseError(format!("{}", e))
    }
}
