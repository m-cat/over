//! Module for parse errors.

#![allow(missing_docs)]

use super::misc::format_char;
use super::ParseResult;
use super::MAX_DEPTH;
use crate::types::Type;
use crate::OverError;
use num_bigint::{BigInt, ParseBigIntError};
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

pub fn parse_err<T>(file: Option<String>, kind: ParseErrorKind) -> ParseResult<T> {
    Err(ParseError { file, kind })
}

/// Error kind.
#[derive(Debug)]
pub enum ParseErrorKind {
    BinaryOperatorError(Type, Type, char, usize, usize),
    CyclicInclude(String, usize, usize),
    DuplicateField(String, usize, usize),
    DuplicateGlobal(String, usize, usize),
    ExpectedType(Type, Type, usize, usize),
    GlobalNotFound(String, usize, usize),
    InvalidIndex(BigInt, usize, usize),
    InvalidClosingBracket(Option<char>, char, usize, usize),
    InvalidDot(Type, usize, usize),
    InvalidEscapeChar(char, usize, usize),
    InvalidFieldChar(char, usize, usize),
    InvalidFieldName(String, usize, usize),
    InvalidIncludeChar(char, usize, usize),
    InvalidIncludePath(String, usize, usize),
    InvalidIncludeToken(Type, usize, usize),
    InvalidNumeric(usize, usize),
    InvalidValue(String, usize, usize),
    InvalidValueChar(char, usize, usize),
    MaxDepth(usize, usize),
    UnaryOperatorError(Type, char, usize, usize),
    UnexpectedEnd(usize),
    VariableNotFound(String, usize, usize),

    IoError(String),
    OverError(String),
    ParseIntError(String),
}

/// Parse error.
#[derive(Debug)]
pub struct ParseError {
    /// The file this error occurred in.
    pub file: Option<String>,
    /// Error kind.
    pub kind: ParseErrorKind,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseErrorKind::*;

        if let Some(ref file) = (*self).file {
            write!(f, "{}: ", file)?;
        }

        match (*self).kind {
            BinaryOperatorError(ref expected, ref found, ref op, ref line, ref col) => write!(
                f,
                "Could not apply operator {} on types {} and {} at line {}, column {}",
                op, expected, found, line, col,
            ),
            CyclicInclude(ref file, ref line, ref col) => write!(
                f,
                "Tried to cyclically include file \"{}\" at line {}, column {}",
                file, line, col
            ),
            DuplicateField(ref field, ref line, ref col) => write!(
                f,
                "Duplicate field \"{}\" at line {}, column {}",
                field, line, col
            ),
            DuplicateGlobal(ref field, ref line, ref col) => write!(
                f,
                "Duplicate global \"{}\" at line {}, column {}",
                field, line, col
            ),
            ExpectedType(ref expected, ref found, ref line, ref col) => write!(
                f,
                "Expected {} at line {}, column {}; found {}",
                expected, line, col, found
            ),
            GlobalNotFound(ref var, ref line, ref col) => write!(
                f,
                "Global \"{}\" at line {}, column {} could not be found",
                var, line, col
            ),
            InvalidClosingBracket(ref expected, ref found, ref line, ref col) => write!(
                f,
                "Invalid closing bracket '{}' at line {}, column {}; expected {}",
                found,
                line,
                col,
                match *expected {
                    Some(ch) => format!("'{}'", ch),
                    None => String::from("none"),
                }
            ),
            InvalidDot(ref t, ref line, ref col) => write!(
                f,
                "Invalid use of dot notation on value of type {} at line {}, column {}; \
                 value must be an Obj, Arr, or Tup.",
                t, line, col
            ),
            InvalidEscapeChar(ref ch, ref line, ref col) => write!(
                f,
                "Invalid escape character '\\{}' at line {}, column {}. \
                 If you meant to write a backslash, use '\\\\'",
                format_char(*ch),
                line,
                col
            ),
            InvalidFieldChar(ref ch, ref line, ref col) => write!(
                f,
                "Invalid character '{}' for field at line {}, column {}",
                format_char(*ch),
                line,
                col
            ),
            InvalidFieldName(ref field, ref line, ref col) => write!(
                f,
                "Invalid field name \"{}\" at line {}, column {}",
                field, line, col
            ),
            InvalidIncludeChar(ref found, ref line, ref col) => write!(
                f,
                "Invalid include token character \'{}\' at line {}, column {}",
                found, line, col
            ),
            InvalidIncludePath(ref path, ref line, ref col) => write!(
                f,
                "Invalid include path \"{}\" at line {}, column {}",
                path, line, col
            ),
            InvalidIncludeToken(ref t, ref line, ref col) => write!(
                f,
                "Invalid value of type \"{}\" at line {}, column {}; \
                 must be either a Str value or one of the tokens \
                 \"Obj\", \"Arr\", \"Tup\", or \"Str\"",
                t, line, col
            ),
            InvalidIndex(ref index, ref line, ref col) => write!(
                f,
                "Invalid index {} at line {}, column {}",
                index, line, col
            ),
            InvalidNumeric(ref line, ref col) => {
                write!(f, "Invalid numeric value at line {}, column {}", line, col)
            }
            InvalidValue(ref value, ref line, ref col) => write!(
                f,
                "Invalid value \"{}\" at line {}, column {}",
                value, line, col
            ),
            InvalidValueChar(ref ch, ref line, ref col) => write!(
                f,
                "Invalid character '{}' for value at line {}, column {}",
                format_char(*ch),
                line,
                col
            ),
            MaxDepth(ref line, ref col) => write!(
                f,
                "Exceeded maximum recursion depth ({}) at line {}, column {}",
                MAX_DEPTH, line, col
            ),
            UnaryOperatorError(ref found, ref op, ref line, ref col) => write!(
                f,
                "Could not apply operator {} on type {} at line {}, column {}",
                op, found, line, col,
            ),
            UnexpectedEnd(ref line) => write!(f, "Unexpected end at line {}", line,),
            VariableNotFound(ref var, ref line, ref col) => write!(
                f,
                "Variable \"{}\" at line {}, column {} could not be found",
                var, line, col
            ),

            IoError(ref error) | OverError(ref error) | ParseIntError(ref error) => {
                write!(f, "{}", error)
            }
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseErrorKind::*;

        match (*self).kind {
            BinaryOperatorError(_, _, _, _, _) | UnaryOperatorError(_, _, _, _) => {
                "Could not apply operator"
            }

            CyclicInclude(_, _, _) => "Tried to cyclically include file",
            DuplicateField(_, _, _) => "Duplicate field",
            DuplicateGlobal(_, _, _) => "Duplicate global",
            ExpectedType(_, _, _, _) => "Expected different type",
            GlobalNotFound(_, _, _) => "Global could not be found",
            InvalidClosingBracket(_, _, _, _) => "Invalid closing bracket",
            InvalidDot(_, _, _) => "Invalid use of dot notation",
            InvalidEscapeChar(_, _, _) => "Invalid escape character",
            InvalidFieldChar(_, _, _) => "Invalid character for field",
            InvalidFieldName(_, _, _) => "Invalid field name",
            InvalidIncludeChar(_, _, _) => "Invalid include character",
            InvalidIncludePath(_, _, _) => "Invalid include path",
            InvalidIncludeToken(_, _, _) => "Invalid include token",
            InvalidIndex(_, _, _) => "Invalid index",
            InvalidNumeric(_, _) => "Invalid numeric value",
            InvalidValue(_, _, _) => "Invalid value",
            InvalidValueChar(_, _, _) => "Invalid character for value",
            MaxDepth(_, _) => "Exceeded maximum depth for a container",
            UnexpectedEnd(_) => "Unexpected end when reading value",
            VariableNotFound(_, _, _) => "Variable could not be found",

            IoError(ref error) | OverError(ref error) | ParseIntError(ref error) => error,
        }
    }
}

impl ParseError {
    /// Convert an `OverError` to a `ParseError` given line and column numbers.
    pub fn from_over(e: &OverError, file: Option<String>, line: usize, col: usize) -> Self {
        ParseError {
            file,
            kind: ParseErrorKind::OverError(format!("{} at line {}, col {}", e, line, col)),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError {
            file: None,
            kind: ParseErrorKind::IoError(format!("{}", e)),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError {
            file: None,
            kind: ParseErrorKind::ParseIntError(format!("{}", e)),
        }
    }
}

impl From<ParseBigIntError> for ParseError {
    fn from(e: ParseBigIntError) -> Self {
        ParseError {
            file: None,
            kind: ParseErrorKind::ParseIntError(format!("{}", e)),
        }
    }
}
