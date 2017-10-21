//! Functions for loading/writing Objs.

pub mod error;

mod char_stream;
mod parser;
use self::error::ParseError;

use {Obj, OverError, OverResult};
use std::fs::File;

type ParseResult<T> = Result<T, ParseError>;

pub fn load_file(path: &str) -> OverResult<Obj> {
    parser::parse_file_obj(path).map_err(OverError::from)
}

pub fn write_to_file(obj: &Obj, path: &str) -> OverResult<()> {
    let file = File::open(path).map_err(OverError::from)?;

    unimplemented!()
}
