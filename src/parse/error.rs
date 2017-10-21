//! Module for parse errors.

#![allow(missing_docs)]
#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Parse error type.
#[derive(Debug)]
pub enum ParseError {
    DuplicateField(String, usize, usize),
    InvalidEscapeChar(char, usize, usize),
    InvalidFieldChar(usize, usize),
    InvalidFieldName(String, usize, usize),
    InvalidNumeric(usize, usize),
    InvalidValueChar(usize, usize),
    IoError(String),
    NoWhitespaceAfterField(usize, usize),
    ParseIntError(String),
    UnexpectedEnd(usize, usize),
    UnknownError,
    VariableNotFound(String, usize, usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;

        match *self {
            DuplicateField(ref field, ref line, ref col) => {
                write!(
                    f,
                    "Duplicate field {} found at line {}, column {}",
                    field,
                    line,
                    col
                )
            }
            InvalidEscapeChar(ref ch, ref line, ref col) => {
                write!(
                    f,
                    "Invalid escape character '\\{}' found at line {}, column {}. \
                     If you meant to write a backslash, use '\\\\'",
                    ch,
                    line,
                    col
                )
            }
            InvalidFieldChar(ref line, ref col) => {
                write!(
                    f,
                    "Invalid character for field at line {}, column {}",
                    line,
                    col
                )
            }
            InvalidFieldName(ref field, ref line, ref col) => {
                write!(
                    f,
                    "Invalid field name \"{}\" at line {}, column {}",
                    field,
                    line,
                    col
                )
            }
            InvalidNumeric(ref line, ref col) => {
                write!(
                    f,
                    "Invalid character for numeric value at line {}, column {}",
                    line,
                    col
                )
            }
            InvalidValueChar(ref line, ref col) => {
                write!(
                    f,
                    "Invalid character for value at line {}, column {}",
                    line,
                    col
                )
            }
            IoError(ref error) => write!(f, "{}", error),
            NoWhitespaceAfterField(ref line, ref col) => {
                write!(
                    f,
                    "No whitespace found after field at line {}, column {}",
                    line,
                    col
                )
            }
            ParseIntError(ref error) => write!(f, "{}", error),
            UnexpectedEnd(ref line, ref col) => {
                write!(
                    f,
                    "Unexpected end of file when expecting value at line {}, column {}",
                    line,
                    col
                )
            }
            UnknownError => write!(f, "An unknown error has occurred"),
            VariableNotFound(ref var, ref line, ref col) => {
                write!(
                    f,
                    "Variable \"{}\" at line {}, column {} could not be found",
                    var,
                    line,
                    col
                )
            }
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;

        match *self {
            DuplicateField(_, _, _) => "Duplicate field found",
            InvalidEscapeChar(_, _, _) => "Invalid escape character found",
            InvalidFieldChar(_, _) => "Invalid character for field",
            InvalidFieldName(_, _, _) => "Invalid field name",
            InvalidNumeric(_, _) => "Invalid character for numeric value",
            InvalidValueChar(_, _) => "Invalid character for value",
            IoError(ref error) => error,
            NoWhitespaceAfterField(_, _) => "No whitespace found after field",
            ParseIntError(ref error) => error,
            UnexpectedEnd(_, _) => "Unexpected end of file when expecting value",
            UnknownError => "An unknown error has occurred",
            VariableNotFound(_, _, _) => "Variable could not be found",
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::IoError(format!("{}", e))
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::ParseIntError(format!("{}", e))
    }
}
