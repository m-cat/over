//! Module containing parsing functions.

use std::fs::File;
use std::io::BufReader;
use self::error::ParseError;
use self::ParseResult;
use obj::Obj;

pub struct Parser {
    contents: String,
    pos: usize,
}

impl Parser {
    pub fn from_str(s: &str) -> ParseResult<Parser> {
        Parser {
            contents: String::from(s),
            pos: 0,
        }
    }

    pub fn from_file(path: &str) -> ParseResult<Parser> {
        let file = File::open(path).map_err(ParseError::from)?;
        let reader = BufReader::new(file);
        let mut contents = String::new();

        reader.read_to_string(&mut contents);

        Parser {
            contents: contents,
            pos: 0,
        }
    }

    pub fn parse_obj() -> ParseResult<Obj> {
        let mut obj = Obj::new();

        while !self.eof() {
            let field = self.parse_field()?;
            if obj.contains(field) {
                return Err(ParseError::DuplicateField);
            }

            let value = self.parse_value()?;

            obj.set(field, value);
        }

        Ok(obj)
    }
}
