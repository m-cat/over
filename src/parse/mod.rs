//! Functions for loading/writing Objs.

pub mod error;
pub mod format;

mod char_stream;
mod parser;
mod util;

use self::error::ParseError;
use crate::Obj;
use std::fmt;

type ParseResult<T> = Result<T, ParseError>;

const MAX_DEPTH: usize = 64;

/// Load an `Obj` from a file.
pub fn load_from_file(path: &str) -> ParseResult<Obj> {
    parser::parse_obj_file(path)
}

/// Load an `Obj` from a &str.
pub fn load_from_str(contents: &str) -> ParseResult<Obj> {
    parser::parse_obj_str(contents)
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}'",
            match *self {
                UnaryOp::Plus => '+',
                UnaryOp::Minus => '-',
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
}

impl BinaryOp {
    pub fn is_priority(&self) -> bool {
        match *self {
            BinaryOp::Mult | BinaryOp::Div | BinaryOp::Mod => true,
            _ => false,
        }
    }

    /// Is this a binary operator?
    pub fn is_op(ch: char) -> bool {
        match ch {
            '+' | '-' | '*' | '/' | '%' => true,
            _ => false,
        }
    }

    pub fn get_op(ch: char) -> Option<Self> {
        Some(match ch {
            '+' => BinaryOp::Plus,
            '-' => BinaryOp::Minus,
            '*' => BinaryOp::Mult,
            '/' => BinaryOp::Div,
            '%' => BinaryOp::Mod,
            _ => return None,
        })
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "'{}'",
            match *self {
                BinaryOp::Plus => '+',
                BinaryOp::Minus => '-',
                BinaryOp::Mult => '*',
                BinaryOp::Div => '/',
                BinaryOp::Mod => '%',
            }
        )
    }
}
