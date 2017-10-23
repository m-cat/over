//! Module containing parsing functions.

use super::ParseResult;
use super::char_stream::CharStream;
use super::error::ParseError;
use obj::Obj;
use std::collections::HashMap;
use value::Value;

/// Parse given file as an `Obj`.
pub fn parse_file_obj(path: &str) -> ParseResult<Obj> {
    let stream = CharStream::from_file(path).map_err(ParseError::from)?;

    let mut obj = Obj::new();
    let mut globals: HashMap<String, Value> = HashMap::new();

    while find_char(stream.clone()) {
        parse_field_value_pair(stream.clone(), &mut obj, &mut globals)?;
    }

    Ok(obj)
}

// Parse a sub-Obj in a file. It *must* start with { and end with }.
fn parse_obj(mut stream: CharStream, globals: &mut HashMap<String, Value>) -> ParseResult<Value> {
    let mut obj = Obj::new();

    let ch = stream.next().unwrap();
    assert_eq!(ch, '{');

    loop {
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
        }

        let peek = stream.peek().unwrap();
        if peek == '}' {
            let _ = stream.next();

            // Values must be followed by either whitespace or eof.
            parse_value_end(stream.clone())?;

            break;
        }

        parse_field_value_pair(stream.clone(), &mut obj, globals)?;
    }

    Ok(Value::Obj(obj))
}

// Parses a field/value pair.
fn parse_field_value_pair(
    stream: CharStream,
    obj: &mut Obj,
    mut globals: &mut HashMap<String, Value>,
) -> ParseResult<()> {
    let (field_line, field_col) = (stream.line(), stream.col());
    // At a non-whitespace character, parse field.
    let (field, is_global) = parse_field(stream.clone(), field_line, field_col)?;
    if obj.contains(&field) {
        return Err(ParseError::DuplicateField(field, field_line, field_col));
    }

    // Deal with extra whitespace between field and value.
    if !find_char(stream.clone()) {
        return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
    }

    // At a non-whitespace character, parse value.
    let (value_line, value_col) = (stream.line(), stream.col());
    let value = parse_value(stream.clone(), &obj, &mut globals, value_line, value_col)?;

    // Add value either to the globals map or to the current Obj.
    if is_global {
        if globals.contains_key(&field) {
            return Err(ParseError::DuplicateGlobal(field, field_line, field_col));
        }
        globals.insert(field, value);
    } else {
        obj.set(&field, value);
    }

    Ok(())
}

// Get the next field in the char stream.
fn parse_field(mut stream: CharStream, line: usize, col: usize) -> ParseResult<(String, bool)> {
    let mut field = String::new();
    let mut first = true;
    let mut is_global = false;

    let ch = stream.peek().unwrap();
    if ch == '@' {
        let ch = stream.next().unwrap();
        is_global = true;
        field.push(ch);
    }

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            break;
        }

        match ch_opt.unwrap() {
            ':' => {
                // There must be whitespace after a field.
                let ch_opt = stream.next();
                if ch_opt.is_some() && !ch_opt.unwrap().is_whitespace() {
                    return Err(ParseError::NoWhitespaceAfterField(
                        stream.line(),
                        stream.col() - 1,
                    ));
                }
                break;
            }
            ch if is_valid_field_char(ch, first) => field.push(ch),
            ch => {
                return Err(ParseError::InvalidFieldChar(
                    ch,
                    stream.line(),
                    stream.col() - 1,
                ))
            }
        }

        first = false;
    }

    // Check for invalid field names.
    match field.as_str() {
        "true" | "false" | "null" | "@" => Err(ParseError::InvalidFieldName(field, line, col)),
        _ => Ok((field, is_global)),
    }
}

// Get the next value in the char stream.
fn parse_value(
    stream: CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    // Peek to determine what kind of value we'll be parsing.
    match stream.peek().unwrap() {
        '"' => parse_str(stream),
        ch if ch.is_numeric() => parse_numeric(stream),
        ch if ch.is_alphabetic() => parse_variable(stream, obj, globals, line, col),
        '@' => parse_variable(stream, obj, globals, line, col),
        '{' => parse_obj(stream, &mut globals),
        ch => {
            return Err(ParseError::InvalidValueChar(
                ch,
                stream.line(),
                stream.col(),
            ))
        }
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
fn parse_variable(
    mut stream: CharStream,
    obj: &Obj,
    globals: &HashMap<String, Value>,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    let mut var = String::new();
    let mut is_global = false;

    let ch = stream.peek().unwrap();
    if ch == '@' {
        let ch = stream.next().unwrap();
        is_global = true;
        var.push(ch);
    }

    loop {
        let ch_opt = stream.next();
        if ch_opt.is_none() {
            break;
        }

        match ch_opt.unwrap() {
            ch if ch.is_whitespace() => break,
            ch if is_valid_field_char(ch, false) => var.push(ch),
            ch => {
                return Err(ParseError::InvalidValueChar(
                    ch,
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
        var @ "@" => Err(ParseError::InvalidValue(var.into(), line, col)),
        var if is_global => {
            // Global variable, get value from globals map.
            match globals.get(var) {
                Some(value) => Ok(value.clone()),
                None => {
                    let var = String::from(var);
                    Err(ParseError::GlobalNotFound(var, line, col))
                }
            }
        }
        var => {
            // Regular variable, get value from the current Obj.
            match obj.get(var) {
                Some(value) => Ok(value),
                None => {
                    let var = String::from(var);
                    Err(ParseError::VariableNotFound(var, line, col))
                }
            }
        }
    }
}

// Get the next Str in the character stream.
// Assumes the Str starts and ends with quotation marks and removes them.
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

    // There must be whitespace after the string.
    parse_value_end(stream.clone())?;

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

// Helper function to make sure values are followed by whitespace.
fn parse_value_end(mut stream: CharStream) -> ParseResult<()> {
    let ch_opt = stream.next();
    if ch_opt.is_some() && !ch_opt.unwrap().is_whitespace() {
        Err(ParseError::InvalidValueChar(
            ch_opt.unwrap(),
            stream.line(),
            stream.col() - 1,
        ))
    } else {
        Ok(())
    }
}

// Returns true if the given char is valid for a field, given whether it is the first char or not.
// The first character must be alphabetic.
// Subsequent characters are allowed to be alphabetic, numeric, or '_'.
fn is_valid_field_char(ch: char, first: bool) -> bool {
    match ch {
        ch if ch.is_alphabetic() => true,
        ch if ch.is_numeric() => !first,
        '_' => !first,
        _ => false,
    }
}
