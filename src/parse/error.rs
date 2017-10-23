//! Module for parse errors.

#![allow(missing_docs)]

use super::misc::format_char;
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

/// Parse error type.
#[derive(Debug)]
pub enum ParseError {
    DuplicateField(String, usize, usize),
    DuplicateGlobal(String, usize, usize),
    GlobalNotFound(String, usize, usize),
    InvalidEscapeChar(char, usize, usize),
    InvalidFieldChar(char, usize, usize),
    InvalidFieldName(String, usize, usize),
    InvalidNumeric(usize, usize),
    InvalidValue(String, usize, usize),
    InvalidValueChar(char, usize, usize),
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
                    "Duplicate field \"{}\" at line {}, column {}",
                    field,
                    line,
                    col
                )
            }
            DuplicateGlobal(ref field, ref line, ref col) => {
                write!(
                    f,
                    "Duplicate global \"{}\" at line {}, column {}",
                    field,
                    line,
                    col
                )
            }
            GlobalNotFound(ref var, ref line, ref col) => {
                write!(
                    f,
                    "Global \"{}\" at line {}, column {} could not be found",
                    var,
                    line,
                    col
                )
            }
            InvalidEscapeChar(ref ch, ref line, ref col) => {
                write!(
                    f,
                    "Invalid escape character '\\{}' at line {}, column {}. \
                     If you meant to write a backslash, use '\\\\'",
                    &format_char(ch),
                    line,
                    col
                )
            }
            InvalidFieldChar(ref ch, ref line, ref col) => {
                write!(
                    f,
                    "Invalid character '{}' for field at line {}, column {}",
                    &format_char(ch),
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
            InvalidValue(ref value, ref line, ref col) => {
                write!(
                    f,
                    "Invalid value \"{}\" at line {}, column {}",
                    value,
                    line,
                    col
                )
            }
            InvalidValueChar(ref ch, ref line, ref col) => {
                write!(
                    f,
                    "Invalid character '{}' for value at line {}, column {}",
                    &format_char(ch),
                    line,
                    col
                )
            }
            IoError(ref error) => write!(f, "{}", error),
            NoWhitespaceAfterField(ref line, ref col) => {
                write!(
                    f,
                    "No whitespace after field at line {}, column {}",
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
            DuplicateField(_, _, _) => "Duplicate field",
            DuplicateGlobal(_, _, _) => "Duplicate global",
            GlobalNotFound(_, _, _) => "Global could not be found",
            InvalidEscapeChar(_, _, _) => "Invalid escape character",
            InvalidFieldChar(_, _, _) => "Invalid character for field",
            InvalidFieldName(_, _, _) => "Invalid field name",
            InvalidNumeric(_, _) => "Invalid character for numeric value",
            InvalidValue(_, _, _) => "Invalid value",
            InvalidValueChar(_, _, _) => "Invalid character for value",
            IoError(ref error) => error,
            NoWhitespaceAfterField(_, _) => "No whitespace after field",
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
