//! Functions for loading/writing Objs.

pub mod error;
// mod parser;

// use {Obj, OverError, OverResult};
// use self::error::ParseError;

// type ParseResult<T> = Result<T, ParseError>;

// pub fn load_file(path: &str) -> OverResult<Obj> {
//     let parser = Parser::from_file(path).map_err(OverError::from)?;

//     parser.parse_obj().map_err(OverError::from)
// }

// pub fn write_to_file(obj: &Obj, path: &str) -> OverResult<()> {
//     let file = File::open(path).map_err(OverError::from)?;

//     Ok(())
// }
