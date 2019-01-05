//! Functions for loading/writing Objs.

pub mod error;
pub mod format;

mod char_stream;
mod misc;
mod parser;
mod util;

use self::error::ParseError;
use crate::Obj;

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
