//! Error module.

#![allow(missing_docs)]

use parse::error::ParseError;
use std::error::Error;
use std::fmt;
use types::Type;

/// The fabulous OVER error type.
#[derive(Debug, PartialEq, Eq)]
pub enum OverError {
    ArrOutOfBounds(usize),
    ArrTypeMismatch(Type, Type),
    CircularParentReferences,
    NoParentFound,
    ParseError(String),
    TupOutOfBounds(usize),
    TypeMismatch(Type),
}

impl fmt::Display for OverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OverError::*;

        match *self {
            ArrOutOfBounds(ref index) => write!(f, "Arr index out of bounds: {}", index),
            ArrTypeMismatch(ref found, ref expected) => {
                write!(
                    f,
                    "Arr inner types do not match: found {}, expected {}",
                    found,
                    expected
                )
            }
            CircularParentReferences => {
                write!(f, "Circular references among parents are not allowed")
            }
            NoParentFound => write!(f, "No parent found for this obj"),
            ParseError(ref error) => write!(f, "{}", error),
            TupOutOfBounds(ref index) => write!(f, "Tup index out of bounds: {}", index),
            TypeMismatch(ref found) => write!(f, "Type mismatch: found {}", found),
        }
    }
}

impl Error for OverError {
    fn description(&self) -> &str {
        use self::OverError::*;

        match *self {
            ArrOutOfBounds(_) => "Arr index out of bounds",
            ArrTypeMismatch(_, _) => "Arr inner types do not match",
            CircularParentReferences => "Circular references among parents are not allowed",
            NoParentFound => "No parent found for this obj",
            ParseError(ref error) => error,
            TupOutOfBounds(_) => "Tup index out of bounds",
            TypeMismatch(_) => "Type mismatch",
        }
    }
}

impl From<ParseError> for OverError {
    fn from(e: ParseError) -> Self {
        OverError::ParseError(format!("{}", e))
    }
}
