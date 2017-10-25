//! Module containing parsing functions.

use super::ParseResult;
use super::char_stream::CharStream;
use super::error::ParseError;
use arr::Arr;
use obj::Obj;
use std::collections::HashMap;
use tup::Tup;
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
    let ch = stream.next().unwrap();
    assert_eq!(ch, '{');

    let mut obj = Obj::new();

    loop {
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
        }

        let peek = stream.peek().unwrap();
        if peek == '}' {
            let _ = stream.next();
            break;
        }

        parse_field_value_pair(stream.clone(), &mut obj, globals)?;
    }

    // Check for valid characters after the Obj.
    check_value_end(stream.clone())?;

    Ok(obj.into())
}

// Parses a field/value pair.
fn parse_field_value_pair(
    stream: CharStream,
    obj: &mut Obj,
    mut globals: &mut HashMap<String, Value>,
) -> ParseResult<()> {
    let (field_line, field_col) = (stream.line(), stream.col());
    // At a non-whitespace character, parse field.
    let (field, is_global, is_parent) = parse_field(stream.clone(), field_line, field_col)?;
    if !is_global && !is_parent && obj.contains(&field) {
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
        if is_parent {
            let parent = value.get_obj().map_err(ParseError::from)?;
            obj.set_parent(&parent).map_err(ParseError::from)?;
        } else {
            obj.set(&field, value);
        }
    }

    Ok(())
}

fn parse_arr(
    mut stream: CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
) -> ParseResult<Value> {
    let ch = stream.next().unwrap();
    assert_eq!(ch, '[');

    let mut arr = Arr::new();

    loop {
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
        }

        let peek = stream.peek().unwrap();
        if peek == ']' {
            let _ = stream.next();
            break;
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(stream.clone(), &obj, &mut globals, value_line, value_col)?;

        arr.push(value).map_err(ParseError::from)?;
    }

    // Check for valid characters after the Arr.
    check_value_end(stream.clone())?;

    Ok(arr.into())
}

fn parse_tup(
    mut stream: CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
) -> ParseResult<Value> {
    let ch = stream.next().unwrap();
    assert_eq!(ch, '(');

    let mut vec = Vec::new();

    loop {
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1));
        }

        let peek = stream.peek().unwrap();
        if peek == ')' {
            let _ = stream.next();
            break;
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(stream.clone(), &obj, &mut globals, value_line, value_col)?;

        vec.push(value);
    }

    // Check for valid characters after the Tup.
    check_value_end(stream.clone())?;

    let tup = Tup::from_vec(vec);

    Ok(tup.into())
}

// Get the next field in the char stream.
fn parse_field(
    mut stream: CharStream,
    line: usize,
    col: usize,
) -> ParseResult<(String, bool, bool)> {
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
        match stream.next() {
            Some(ch) => {
                match ch {
                    ':' => {
                        // There must be whitespace after a field.
                        let peek_opt = stream.peek();
                        if peek_opt.is_some() && !is_whitespace(peek_opt.unwrap()) {
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
            }
            None => break,
        }

        first = false;
    }

    // Check for invalid field names.
    match field.as_str() {
        "true" | "false" | "null" | "@" => Err(
            ParseError::InvalidFieldName(field.clone(), line, col),
        ),
        "^" => Ok((field.clone(), false, true)),
        bad if bad.starts_with("^") => Err(ParseError::InvalidFieldName(field.clone(), line, col)),
        _ => Ok((field.clone(), is_global, false)),
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
        '\'' => parse_char(stream),
        ch if ch.is_numeric() => parse_numeric(stream),
        ch if ch.is_alphabetic() => parse_variable(stream, obj, globals, line, col),
        '@' => parse_variable(stream, obj, globals, line, col),
        '{' => parse_obj(stream, &mut globals),
        '[' => parse_arr(stream, obj, &mut globals),
        '(' => parse_tup(stream, obj, &mut globals),
        ch => {
            Err(ParseError::InvalidValueChar(
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
        match stream.peek() {
            Some(ch) => {
                match ch {
                    ch if is_value_end_char(ch) => break,
                    ch if ch.is_numeric() => s.push(ch),
                    _ => return Err(ParseError::InvalidNumeric(stream.line(), stream.col() - 1)),
                }
            }
            None => break,
        }

        let _ = stream.next();
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
        match stream.peek() {
            Some(ch) => {
                match ch {
                    ch if is_value_end_char(ch) => break,
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
            None => break,
        }

        let _ = stream.next();
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

// Get the next Char in the character stream.
// Assumes the Char starts and ends with single quote marks.
// '\', '\n', '\r', and '\t' must be escaped with '\'.
// ''' do not need to be escaped, although they can be.
fn parse_char(mut stream: CharStream) -> ParseResult<Value> {
    let ch = stream.next().unwrap();
    assert_eq!(ch, '\'');

    let (escape, mut ch) = match stream.next() {
        Some('\\') => (true, '\0'),
        Some(ch) if ch == '\n' || ch == '\r' || ch == '\t' => {
            return Err(ParseError::InvalidValueChar(
                ch,
                stream.line(),
                stream.col() - 1,
            ))
        }
        Some(ch) => (false, ch),
        None => return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1)),
    };

    if escape {
        ch = match stream.next() {
            Some(ch) => {
                match escape_char(ch) {
                    Some(ch) => ch,
                    None => {
                        return Err(ParseError::InvalidEscapeChar(
                            ch,
                            stream.line(),
                            stream.col() - 1,
                        ))
                    }
                }
            }
            None => return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1)),
        }
    }

    match stream.next() {
        Some('\'') => (),
        Some(ch) => {
            return Err(ParseError::InvalidValueChar(
                ch,
                stream.line(),
                stream.col() - 1,
            ))
        }
        None => return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1)),
    }

    // Check for valid characters after the char.
    check_value_end(stream.clone())?;

    Ok(ch.into())
}

// Get the next Str in the character stream.
// Assumes the Str starts and ends with quotation marks and does not include them in the Str.
// '"', '\' and '$' must be escaped with '\'.
// Newlines can be escaped with '\n', but this is NOT necessary.
fn parse_str(mut stream: CharStream) -> ParseResult<Value> {
    let ch = stream.next().unwrap();
    assert_eq!(ch, '"');

    let mut s = String::new();
    let mut escape = false;

    loop {
        match stream.next() {
            Some(ch) => {
                if escape {
                    match escape_char(ch) {
                        Some(ch) => s.push(ch),
                        None => {
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
            None => return Err(ParseError::UnexpectedEnd(stream.line(), stream.col() - 1)),
        }
    }

    // Check for valid characters after the string.
    check_value_end(stream.clone())?;

    Ok(s.into())
}

// Find the next non-whitespace character, ignoring comments, and update stream position.
// Return true if such a character was found or false if we got to the end of the stream.
fn find_char(mut stream: CharStream) -> bool {
    loop {
        match stream.peek() {
            Some(ch) => {
                match ch {
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
            None => break,
        }
    }

    false
}

// Helper function to make sure values are followed by whitespace or an end delimiter.
fn check_value_end(stream: CharStream) -> ParseResult<()> {
    match stream.peek() {
        Some(ch) => {
            match ch {
                ch if is_value_end_char(ch) => Ok(()),
                ch => Err(ParseError::InvalidValueChar(
                    ch,
                    stream.line(),
                    stream.col() - 1,
                )),
            }
        }
        None => Ok(()),
    }
}

// If `ch` preceded by a backslash together form an escape character, then return this char.
// Otherwise, return None.
fn escape_char(ch: char) -> Option<char> {
    match ch {
        '"' => Some('"'),
        '\'' => Some('\''),
        '\\' => Some('\\'),
        '$' => Some('$'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        _ => None,
    }
}

fn is_value_end_char(ch: char) -> bool {
    is_whitespace(ch) || is_end_delimiter(ch)
}

// Returns true if the character is either whitespace or '#' (start of a comment).
fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == '#'
}

fn is_end_delimiter(ch: char) -> bool {
    match ch {
        ')' => true,
        ']' => true,
        '}' => true,
        _ => false,
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
        '^' => first,
        _ => false,
    }
}
