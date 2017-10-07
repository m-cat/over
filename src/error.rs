//! Error module.

#![allow(missing_docs)]

use std::error::Error;
use std::fmt;
use std::io;

/// The fabulous OVER error type.
#[derive(Debug, PartialEq, Eq)]
pub enum OverError {
    ArrOutOfBounds(usize),
    ArrTypeMismatch,
    CircularParentReferences,
    FieldNotFound(String),
    IoError(String),
    NoParentFound,
    NullError,
    ParseError(String),
    SyncError,
    TupOutOfBounds(usize),
    TupTypeMismatch,
    TypeMismatch,
    UnknownError,
}

impl fmt::Display for OverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OverError::*;

        match *self {
            ArrOutOfBounds(ref index) => write!(f, "Arr index out of bounds: {}", index),
            ArrTypeMismatch => write!(f, "Arr inner types do not match"),
            CircularParentReferences => {
                write!(f, "Circular references among parents are not allowed")
            }
            FieldNotFound(ref field) => write!(f, "Field not found: {}", field),
            IoError(ref field) => write!(f, "{}", field),
            NoParentFound => write!(f, "No parent found for this obj"),
            NullError => write!(f, "Tried to access a null value"),
            ParseError(ref field) => write!(f, "{}", field),
            SyncError => write!(f, "Tried to access two values at the same time"),
            TupOutOfBounds(ref index) => write!(f, "Tup index out of bounds: {}", index),
            TupTypeMismatch => write!(f, "Tup inner types do not match"),
            TypeMismatch => write!(f, "Type mismatch"),
            UnknownError => write!(f, "An unknown error has occurred"),
        }
    }
}

impl Error for OverError {
    fn description(&self) -> &str {
        use self::OverError::*;

        match *self {
            ArrOutOfBounds(_) => "Arr index out of bounds",
            ArrTypeMismatch => "Arr inner types do not match",
            CircularParentReferences => "Circular references among parents are not allowed",
            FieldNotFound(_) => "Field not found: {}",
            IoError(ref field) => field,
            NoParentFound => "No parent found for this obj",
            NullError => "Tried to access a null value",
            ParseError(ref field) => field,
            SyncError => "Tried to access two values at the same time",
            TupOutOfBounds(_) => "Tup index out of bounds",
            TupTypeMismatch => "Tup inner types do not match",
            TypeMismatch => "Type mismatch",
            UnknownError => "An unknown error has occurred",
        }
    }
}

impl From<io::Error> for OverError {
    fn from(e: io::Error) -> Self {
        OverError::IoError(e.description().to_owned())
    }
}
