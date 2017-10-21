//! Module containing parsing functions.

use super::ParseResult;
use super::char_stream::CharStream;
use super::error::ParseError;
use obj::Obj;
use value::Value;

/// Parse given file as an `Obj`.
pub fn parse_file_obj(path: &str) -> ParseResult<Obj> {
    let stream = CharStream::from_file(path).map_err(ParseError::from)?;

    let mut obj = Obj::new();

    while find_char(stream.clone()) {
        // At a non-whitespace character, parse field.
        let field = parse_field(stream.clone())?;
        if obj.contains(&field) {
            let len = field.len();
            return Err(ParseError::DuplicateField(
                field,
                stream.line(),
                stream.col() - len,
            ));
        }

        // There must be whitespace after a field.
        let ch_opt = stream.peek();
        if ch_opt.is_some() && !ch_opt.unwrap().is_whitespace() {
            return Err(ParseError::NoWhitespaceAfterField(
                stream.line(),
                stream.col(),
            ));
        }

        // Deal with extra whitespace between field and value.
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col()));
        }

        // At a non-whitespace character, parse value.
        let value = parse_value(stream.clone(), &obj)?;

        obj.set(&field, value);
    }

    Ok(obj)
}

// Get the next field in the char stream.
fn parse_field(mut stream: CharStream) -> ParseResult<String> {
    let ch = stream.next().unwrap();
    if !ch.is_alphabetic() {
        return Err(ParseError::InvalidFieldChar(
            stream.line(),
            stream.col() - 1,
        ));
    }

    let mut field = String::new();
    field.push(ch);

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            break;
        }

        match ch_opt.unwrap() {
            ':' => return Ok(field),
            ch if ch.is_alphabetic() => field.push(ch),
            '_' => field.push('_'),
            _ => {
                return Err(ParseError::InvalidFieldChar(
                    stream.line(),
                    stream.col() - 1,
                ))
            }
        }
    }

    if field == "true" || field == "false" || field == "null" {
        let len = field.len();
        return Err(ParseError::InvalidFieldName(
            field,
            stream.line(),
            stream.col() - len,
        ));
    }

    Ok(field)
}

// Get the next value in the char stream.
fn parse_value(stream: CharStream, obj: &Obj) -> ParseResult<Value> {
    // Peek to determine what kind of value we'll be parsing.
    match stream.peek().unwrap() {
        '"' => parse_str(stream),
        ch if ch.is_numeric() => parse_numeric(stream),
        ch if ch.is_alphabetic() => parse_variable(stream, obj),
        _ => return Err(ParseError::InvalidValueChar(stream.line(), stream.col())),
    }
}

// Get the next numeric (either Int or Frac) in the character stream.
fn parse_numeric(mut stream: CharStream) -> ParseResult<Value> {
    let mut s = String::new();

    let ch = stream.next().unwrap();
    s.push(ch);

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            break;
        }

        match ch_opt.unwrap() {
            ch if ch.is_whitespace() => break,
            ch if ch.is_numeric() => s.push(ch),
            _ => return Err(ParseError::InvalidNumeric(stream.line(), stream.col() - 1)),
        }
    }

    let i: i64 = s.parse().map_err(ParseError::from)?;
    Ok(i.into())
}

// Parse a variable name and get a value from the corresponding variable.
fn parse_variable(mut stream: CharStream, obj: &Obj) -> ParseResult<Value> {
    let mut var = String::new();

    let ch = stream.next().unwrap();
    var.push(ch);

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            break;
        }

        match ch_opt.unwrap() {
            ch if ch.is_whitespace() => break,
            ch if ch.is_alphabetic() => var.push(ch),
            '_' => var.push('_'),
            _ => {
                return Err(ParseError::InvalidValueChar(
                    stream.line(),
                    stream.col() - 1,
                ))
            }
        }
    }

    match var.as_str() {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        "null" => Ok(Value::Null),
        var => {
            match obj.get(var) {
                Some(value) => Ok(value),
                None => {
                    let var = String::from(var);
                    let len = var.len();
                    Err(ParseError::VariableNotFound(
                        var,
                        stream.line(),
                        stream.col() - len,
                    ))
                }
            }
        }
    }
}

// Get the next Str in the character stream.
// Assumes the Str starts and ends with " and removes them.
// '"', '\' and '$' must be escaped with '\'.
fn parse_str(mut stream: CharStream) -> ParseResult<Value> {
    let mut s = String::new();
    let mut escape = false;

    let ch = stream.next().unwrap();
    assert_eq!(ch, '"');

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
        }

        let ch = ch_opt.unwrap();
        if escape {
            match ch {
                '"' => s.push('"'),
                '\\' => s.push('\\'),
                '$' => s.push('$'),
                _ => {
                    return Err(ParseError::InvalidEscapeChar(
                        ch,
                        stream.line(),
                        stream.col() - 1,
                    ))
                }
            }
            escape = false;
        } else {
            match ch {
                '"' => break,
                '\\' => escape = true,
                _ => s.push(ch),
            }
        }
    }

    Ok(s.into())
}

// Find the next non-whitespace character, ignoring comments, and update stream position.
// Return true if such a character was found or false if we got to the end of the stream.
fn find_char(mut stream: CharStream) -> bool {
    loop {
        let peek_char = stream.peek();
        if peek_char.is_none() {
            break;
        }

        match peek_char.unwrap() {
            '#' => {
                // Comment found; eat the rest of the line.
                loop {
                    let ch = stream.next();
                    if ch.is_none() {
                        return false;
                    }
                    if ch.unwrap() == '\n' {
                        break;
                    }
                }
            }

            ch if ch.is_whitespace() => {
                let _ = stream.next();
            }

            _ => return true,
        }
    }

    false
}
