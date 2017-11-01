//! Module for parse errors.

#![allow(missing_docs)]

use super::MAX_DEPTH;
use super::misc::format_char;
use OverError;
use num::bigint::ParseBigIntError;
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
    InvalidClosingBracket(char, Option<char>, usize, usize),
    InvalidEscapeChar(char, usize, usize),
    InvalidFieldChar(char, usize, usize),
    InvalidFieldName(String, usize, usize),
    InvalidNumeric(usize, usize),
    InvalidValue(String, usize, usize),
    InvalidValueChar(char, usize, usize),
    MaxDepth(usize, usize),
    NoWhitespaceAfterField(usize, usize),
    UnexpectedEnd(usize),
    VariableNotFound(String, usize, usize),

    IoError(String),
    OverError(String),
    ParseIntError(String),
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
            InvalidClosingBracket(ref found, ref expected, ref line, ref col) => {
                write!(
                    f,
                    "Invalid closing bracket '{}' at line {}, column {}; expected {}",
                    found,
                    line,
                    col,
                    match *expected {
                        Some(ch) => format!("'{}'", ch),
                        None => String::from("none"),
                    }
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
                write!(f, "Invalid numeric value at line {}, column {}", line, col)
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
            MaxDepth(ref line, ref col) => {
                write!(
                    f,
                    "Exceeded maximum depth ({}) for a container at line {}, column {}",
                    MAX_DEPTH,
                    line,
                    col
                )
            }
            NoWhitespaceAfterField(ref line, ref col) => {
                write!(
                    f,
                    "No whitespace after field at line {}, column {}",
                    line,
                    col
                )
            }
            UnexpectedEnd(ref line) => {
                write!(
                    f,
                    "Unexpected end when reading value at line {}",
                    line,
                )
            }
            VariableNotFound(ref var, ref line, ref col) => {
                write!(
                    f,
                    "Variable \"{}\" at line {}, column {} could not be found",
                    var,
                    line,
                    col
                )
            }

            IoError(ref error) |
            OverError(ref error) |
            ParseIntError(ref error) => write!(f, "{}", error),
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
            InvalidClosingBracket(_, _, _, _) => "Invalid closing bracket",
            InvalidEscapeChar(_, _, _) => "Invalid escape character",
            InvalidFieldChar(_, _, _) => "Invalid character for field",
            InvalidFieldName(_, _, _) => "Invalid field name",
            InvalidNumeric(_, _) => "Invalid numeric value",
            InvalidValue(_, _, _) => "Invalid value",
            InvalidValueChar(_, _, _) => "Invalid character for value",
            MaxDepth(_, _) => "Exceeded maximum depth for a container",
            NoWhitespaceAfterField(_, _) => "No whitespace after field",
            UnexpectedEnd(_) => "Unexpected end when reading value",
            VariableNotFound(_, _, _) => "Variable could not be found",

            IoError(ref error) |
            OverError(ref error) |
            ParseIntError(ref error) => error,
        }
    }
}

impl ParseError {
    /// Convert an `OverError` to a `ParseError` given line and column numbers.
    pub fn from_over(e: OverError, line: usize, col: usize) -> Self {
        ParseError::OverError(format!("{} at line {}, col {}", e, line, col))
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

impl From<ParseBigIntError> for ParseError {
    fn from(e: ParseBigIntError) -> Self {
        ParseError::ParseIntError(format!("{}", e))
    }
}
