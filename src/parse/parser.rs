//! Module containing parsing functions.

use super::{MAX_DEPTH, ParseResult};
use super::char_stream::CharStream;
use super::error::ParseError;
use super::util::*;
use arr::Arr;
use num::bigint::{BigInt, BigUint};
use obj::Obj;
use std::collections::HashMap;
use tup::Tup;
use value::Value;

/// Parse given file as an `Obj`.
pub fn parse_obj_file(path: &str) -> ParseResult<Obj> {
    let stream = CharStream::from_file(path).map_err(ParseError::from)?;
    parse_obj_stream(stream, 1)
}

/// Parse given &str as an `Obj`.
pub fn parse_obj_str(contents: &str) -> ParseResult<Obj> {
    let contents = String::from(contents);
    let stream = CharStream::from_string(contents).map_err(ParseError::from)?;
    parse_obj_stream(stream, 1)
}

// Parse an Obj given a character stream.
fn parse_obj_stream(mut stream: CharStream, depth: usize) -> ParseResult<Obj> {
    let mut obj = Obj::new();
    let mut globals: HashMap<String, Value> = HashMap::new();

    // Go to the first non-whitespace character, or return if there is none.
    if !find_char(stream.clone()) {
        return Ok(obj);
    }

    // Parse field/value pairs.
    while parse_field_value_pair(&mut stream, &mut obj, &mut globals, depth, None)? {}

    Ok(obj)
}

// Parse a sub-Obj in a file. It *must* start with { and end with }.
fn parse_obj(
    mut stream: &mut CharStream,
    globals: &mut HashMap<String, Value>,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return Err(ParseError::MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '{'.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '{');

    // Go to the first non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return Err(ParseError::UnexpectedEnd(stream.line()));
    }

    let mut obj = Obj::new();

    // Parse field/value pairs.
    while parse_field_value_pair(&mut stream, &mut obj, globals, depth, Some('}'))? {}

    Ok(obj.into())
}

// Parses a field/value pair.
fn parse_field_value_pair(
    mut stream: &mut CharStream,
    obj: &mut Obj,
    mut globals: &mut HashMap<String, Value>,
    depth: usize,
    cur_brace: Option<char>,
) -> ParseResult<bool> {
    // Check if we're at an end delimiter instead of a field.
    let peek = stream.peek().unwrap();
    if peek == '}' && cur_brace.is_some() {
        let _ = stream.next();
        return Ok(false);
    } else if is_end_delimiter(peek) {
        return Err(ParseError::InvalidClosingBracket(
            peek,
            cur_brace,
            stream.line(),
            stream.col(),
        ));
    }

    // Get the field line/col.
    let (field_line, field_col) = (stream.line(), stream.col());

    // Parse field.
    let (field, is_global, is_parent) = parse_field(stream.clone(), field_line, field_col)?;
    if !is_global && !is_parent && obj.contains(&field) {
        return Err(ParseError::DuplicateField(field, field_line, field_col));
    }

    // Deal with extra whitespace between field and value.
    if !find_char(stream.clone()) {
        return Err(ParseError::UnexpectedEnd(stream.line()));
    }

    // At a non-whitespace character, parse value.
    let (value_line, value_col) = (stream.line(), stream.col());
    let value = parse_value(
        &mut stream,
        obj,
        &mut globals,
        value_line,
        value_col,
        depth,
        cur_brace,
    )?;

    // Add value either to the globals map or to the current Obj.
    if is_global {
        if globals.contains_key(&field) {
            return Err(ParseError::DuplicateGlobal(field, field_line, field_col));
        }
        globals.insert(field, value);
    } else if is_parent {
        let parent = value.get_obj().map_err(|e| {
            ParseError::from_over(e, value_line, value_col)
        })?;
        obj.set_parent(&parent).map_err(|e| {
            ParseError::from_over(e, value_line, value_col)
        })?;
    } else {
        obj.set(&field, value);
    }

    // Go to the next non-whitespace character.
    if !find_char(stream.clone()) {
        match cur_brace {
            Some(_) => return Err(ParseError::UnexpectedEnd(stream.line())),
            None => return Ok(false),
        }
    }

    Ok(true)
}

// Parse a sub-Arr in a file. It *must* start with [ and end with ].
fn parse_arr(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return Err(ParseError::MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '['.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '[');

    let mut arr = Arr::new();

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line()));
        }

        let peek = stream.peek().unwrap();
        if peek == ']' {
            let _ = stream.next();
            break;
        } else if is_end_delimiter(peek) {
            return Err(ParseError::InvalidClosingBracket(
                peek,
                Some(']'),
                stream.line(),
                stream.col(),
            ));
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            obj,
            &mut globals,
            value_line,
            value_col,
            depth,
            Some(']'),
        )?;

        arr.push(value).map_err(|e| {
            ParseError::from_over(e, value_line, value_col)
        })?;
    }

    Ok(arr.into())
}

// Parse a sub-Tup in a file. It *must* start with ( and end with ).
fn parse_tup(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return Err(ParseError::MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '('.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '(');

    let mut vec = Vec::new();

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            return Err(ParseError::UnexpectedEnd(stream.line()));
        }

        let peek = stream.peek().unwrap();
        if peek == ')' {
            let _ = stream.next();
            break;
        } else if is_end_delimiter(peek) {
            return Err(ParseError::InvalidClosingBracket(
                peek,
                Some(')'),
                stream.line(),
                stream.col(),
            ));
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            obj,
            &mut globals,
            value_line,
            value_col,
            depth,
            Some(')'),
        )?;

        vec.push(value);
    }

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

    while let Some(ch) = stream.next() {
        match ch {
            ':' if !first => {
                // There must be whitespace after a field.
                let peek_opt = stream.peek();
                if peek_opt.is_some() && !is_whitespace(peek_opt.unwrap()) {
                    return Err(ParseError::NoWhitespaceAfterField(
                        stream.line(),
                        stream.col(),
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
        "true" | "false" | "null" | "@" => Err(
            ParseError::InvalidFieldName(field.clone(), line, col),
        ),
        "^" => Ok((field.clone(), false, true)),
        bad if bad.starts_with('^') => Err(ParseError::InvalidFieldName(field.clone(), line, col)),
        _ => Ok((field.clone(), is_global, false)),
    }
}

// Get the next value in the char stream.
fn parse_value(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut HashMap<String, Value>,
    line: usize,
    col: usize,
    depth: usize,
    cur_brace: Option<char>,
) -> ParseResult<Value> {
    // Peek to determine what kind of value we'll be parsing.
    let res = match stream.peek().unwrap() {
        '"' => parse_str(&mut stream)?,
        '\'' => parse_char(&mut stream)?,
        ch if is_numeric_char(ch) => parse_numeric(&mut stream, line, col)?,
        ch if ch.is_alphabetic() => parse_variable(&mut stream, obj, globals, line, col)?,
        '@' => parse_variable(&mut stream, obj, globals, line, col)?,
        '{' => parse_obj(&mut stream, &mut globals, depth + 1)?,
        '[' => parse_arr(&mut stream, obj, &mut globals, depth + 1)?,
        '(' => parse_tup(&mut stream, obj, &mut globals, depth + 1)?,
        // '<' => parse_include(&mut stream)?,
        ch => {
            return Err(ParseError::InvalidValueChar(ch, line, col));
        }
    };

    // Check for valid characters after the value.
    check_value_end(stream, cur_brace)?;

    Ok(res)
}

// Get the next numeric (either Int or Frac) in the character stream.
fn parse_numeric(stream: &mut CharStream, line: usize, col: usize) -> ParseResult<Value> {
    let mut s1 = String::new();
    let mut s2 = String::new();
    let mut neg1 = false;
    let mut first = true;
    let mut dec = false;

    while let Some(ch) = stream.peek() {
        match ch {
            ch if is_value_end_char(ch) => break,
            ch if is_digit(ch) => {
                if !dec {
                    s1.push(ch);
                } else {
                    s2.push(ch);
                }
            }
            '+' => {
                if !first {
                    break;
                }
            }
            '-' => {
                if !first {
                    break;
                } else {
                    neg1 = true;
                }
            }
            '.' => {
                if !dec {
                    dec = true;
                } else {
                    return Err(ParseError::InvalidValueChar(
                        ch,
                        stream.line(),
                        stream.col(),
                    ));
                }
            }
            _ => {
                return Err(ParseError::InvalidValueChar(
                    ch,
                    stream.line(),
                    stream.col(),
                ))
            }
        }

        let _ = stream.next();
        first = false;
    }

    if dec {
        // Parse a Frac from a number with a decimal.
        if s1.is_empty() && s2.is_empty() {
            return Err(ParseError::InvalidNumeric(line, col));
        }

        let whole: BigUint = if s1.is_empty() {
            0u8.into()
        } else {
            s1.parse().map_err(ParseError::from)?
        };

        // Remove trailing zeros.
        let s2 = s2.trim_right_matches('0');

        let (decimal, dec_len): (BigUint, usize) = if s2.is_empty() {
            (0u8.into(), 1)
        } else {
            (s2.parse().map_err(ParseError::from)?, s2.len())
        };

        let f = frac_from_whole_and_dec(whole, decimal, dec_len, neg1);
        Ok(f.into())
    } else {
        // Parse an Int.
        if s1.is_empty() {
            return Err(ParseError::InvalidNumeric(line, col));
        }

        let mut i: BigInt = s1.parse().map_err(ParseError::from)?;
        if neg1 {
            i = i * -1
        }
        Ok(i.into())
    }
}

// Parse a variable name and get a value from the corresponding variable.
fn parse_variable(
    stream: &mut CharStream,
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

    while let Some(ch) = stream.peek() {
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
fn parse_char(stream: &mut CharStream) -> ParseResult<Value> {
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
        None => return Err(ParseError::UnexpectedEnd(stream.line())),
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
            None => return Err(ParseError::UnexpectedEnd(stream.line())),
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
        None => return Err(ParseError::UnexpectedEnd(stream.line())),
    }

    Ok(ch.into())
}

// Get the next Str in the character stream.
// Assumes the Str starts and ends with quotation marks and does not include them in the Str.
// '"', '\' and '$' must be escaped with '\'.
// Newlines can be escaped with '\n', but this is NOT necessary.
fn parse_str(stream: &mut CharStream) -> ParseResult<Value> {
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
            None => return Err(ParseError::UnexpectedEnd(stream.line())),
        }
    }

    Ok(s.into())
}

// Find the next non-whitespace character, ignoring comments, and update stream position.
// Return true if such a character was found or false if we got to the end of the stream.
fn find_char(mut stream: CharStream) -> bool {
    while let Some(ch) = stream.peek() {
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

    false
}

// Helper function to make sure values are followed by whitespace or a correct end delimiter.
fn check_value_end(stream: &CharStream, cur_brace: Option<char>) -> ParseResult<()> {
    match stream.peek() {
        Some(ch) => {
            match ch {
                ch if is_value_end_char(ch) => {
                    if is_end_delimiter(ch) && Some(ch) != cur_brace {
                        Err(ParseError::InvalidClosingBracket(
                            ch,
                            cur_brace,
                            stream.line(),
                            stream.col(),
                        ))
                    } else {
                        Ok(())
                    }
                }
                ch => Err(ParseError::InvalidValueChar(
                    ch,
                    stream.line(),
                    stream.col(),
                )),
            }
        }
        None => Ok(()),
    }
}
