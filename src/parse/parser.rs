//! Module containing parsing functions.

use super::{MAX_DEPTH, ParseResult};
use super::char_stream::CharStream;
use super::error::{ParseError, parse_err};
use super::error::ParseErrorKind::*;
use super::util::*;
use arr::{self, Arr};
use num::bigint::BigInt;
use num::rational::BigRational;
use num_traits::Zero;
use obj::Obj;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;
use tup::Tup;
use types::Type;
use value::Value;

type Globals = HashMap<String, Value>;

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
    let mut globals: Globals = HashMap::new();

    // Go to the first non-whitespace character, or return if there is none.
    if !find_char(stream.clone()) {
        return Ok(obj);
    }

    // Parse all field/value pairs for this Obj.
    while parse_field_value_pair(&mut stream, &mut obj, &mut globals, depth, None)? {}

    Ok(obj)
}

// Parse a sub-Obj in a file. It *must* start with { and end with }.
fn parse_obj(
    mut stream: &mut CharStream,
    globals: &mut Globals,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return parse_err(stream.file(), MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '{'.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '{');

    // Go to the first non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
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
    mut globals: &mut Globals,
    depth: usize,
    cur_brace: Option<char>,
) -> ParseResult<bool> {
    // Check if we're at an end delimiter instead of a field.
    let peek = stream.peek().unwrap();
    if peek == '}' && cur_brace.is_some() {
        let _ = stream.next();
        return Ok(false);
    } else if is_end_delimiter(peek) {
        return parse_err(
            stream.file(),
            InvalidClosingBracket(peek, cur_brace, stream.line(), stream.col()),
        );
    }

    // Get the field line/col.
    let (field_line, field_col) = (stream.line(), stream.col());

    // Parse field.
    let (field, is_global, is_parent) = parse_field(stream.clone(), field_line, field_col)?;
    if !is_global && !is_parent && obj.contains(&field) {
        return parse_err(stream.file(), DuplicateField(field, field_line, field_col));
    }

    // Deal with extra whitespace between field and value.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
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
        true,
    )?;

    // Add value either to the globals map or to the current Obj.
    if is_global {
        if globals.contains_key(&field) {
            return parse_err(stream.file(), DuplicateGlobal(field, field_line, field_col));
        }
        globals.insert(field, value);
    } else if is_parent {
        let parent = value.get_obj().map_err(|e| {
            ParseError::from_over(e, stream.file(), value_line, value_col)
        })?;
        obj.set_parent(&parent).map_err(|e| {
            ParseError::from_over(e, stream.file(), value_line, value_col)
        })?;
    } else {
        obj.set(&field, value);
    }

    // Go to the next non-whitespace character.
    if !find_char(stream.clone()) {
        match cur_brace {
            Some(_) => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
            None => return Ok(false),
        }
    }

    Ok(true)
}

// Parse a sub-Arr in a file. It *must* start with [ and end with ].
fn parse_arr(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut Globals,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return parse_err(stream.file(), MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '['.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '[');

    let mut arr = Arr::new();

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            return parse_err(stream.file(), UnexpectedEnd(stream.line()));
        }

        let peek = stream.peek().unwrap();
        if peek == ']' {
            let _ = stream.next();
            break;
        } else if is_end_delimiter(peek) {
            return parse_err(
                stream.file(),
                InvalidClosingBracket(peek, Some(']'), stream.line(), stream.col()),
            );
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
            true,
        )?;

        arr.push(value).map_err(|e| {
            ParseError::from_over(e, stream.file(), value_line, value_col)
        })?;
    }

    Ok(arr.into())
}

// Parse a sub-Tup in a file. It *must* start with ( and end with ).
fn parse_tup(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut Globals,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return parse_err(stream.file(), MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '('.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '(');

    let mut vec = Vec::new();

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            return parse_err(stream.file(), UnexpectedEnd(stream.line()));
        }

        let peek = stream.peek().unwrap();
        if peek == ')' {
            let _ = stream.next();
            break;
        } else if is_end_delimiter(peek) {
            return parse_err(
                stream.file(),
                InvalidClosingBracket(peek, Some(')'), stream.line(), stream.col()),
            );
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
            true,
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
                break;
            }
            ch if is_valid_field_char(ch, first) => field.push(ch),
            ch => {
                return parse_err(
                    stream.file(),
                    InvalidFieldChar(ch, stream.line(), stream.col() - 1),
                )
            }
        }

        first = false;
    }

    // Check for invalid field names.
    match field.as_str() {
        "true" | "false" | "null" | "@" => {
            parse_err(stream.file(), InvalidFieldName(field.clone(), line, col))
        }
        "^" => Ok((field.clone(), false, true)),
        bad if bad.starts_with('^') => {
            parse_err(stream.file(), InvalidFieldName(field.clone(), line, col))
        }
        _ => Ok((field.clone(), is_global, false)),
    }
}

// Get the next value in the char stream.
#[allow(unknown_lints)]
#[allow(too_many_arguments)]
fn parse_value(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut Globals,
    line: usize,
    col: usize,
    depth: usize,
    cur_brace: Option<char>,
    is_first: bool,
) -> ParseResult<Value> {
    // Peek to determine what kind of value we'll be parsing.
    let res = match stream.peek().unwrap() {
        '"' => parse_str(&mut stream)?,
        '\'' => parse_char(&mut stream)?,
        '{' => parse_obj(&mut stream, &mut globals, depth + 1)?,
        '[' => parse_arr(&mut stream, obj, &mut globals, depth + 1)?,
        '(' => parse_tup(&mut stream, obj, &mut globals, depth + 1)?,
        '@' => parse_variable(&mut stream, obj, globals, line, col)?,
        '<' => parse_include(&mut stream, obj, &mut globals, depth)?,
        ch @ '+' | ch @ '-' => parse_unary_op(&mut stream, obj, globals, depth, cur_brace, ch)?,
        ch if ch.is_alphabetic() => parse_variable(&mut stream, obj, globals, line, col)?,
        ch if is_numeric_char(ch) => parse_numeric(&mut stream, line, col)?,
        ch => {
            return parse_err(stream.file(), InvalidValueChar(ch, line, col));
        }
    };

    // Process operations if this is the first value.
    if is_first {
        let mut val_deque: VecDeque<(Value, usize, usize)> = VecDeque::new();
        let mut op_deque: VecDeque<char> = VecDeque::new();
        val_deque.push_back((res, line, col));

        loop {
            match stream.peek() {
                Some(ch) if is_operator(ch) => {
                    let _ = stream.next();
                    if stream.peek().is_none() {
                        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
                    }

                    let (line2, col2) = (stream.line(), stream.col());

                    // Parse another value.
                    let val2 = parse_value(
                        &mut stream,
                        obj,
                        &mut globals,
                        line2,
                        col2,
                        depth,
                        cur_brace,
                        false,
                    )?;

                    if is_priority_operator(ch) {
                        let (val1, line1, col1) = val_deque.pop_back().unwrap();
                        let res = binary_op_on_values(stream, val1, val2, ch, line2, col2)?;
                        val_deque.push_back((res, line1, col1));
                    } else {
                        val_deque.push_back((val2, line2, col2));
                        op_deque.push_back(ch);
                    }
                }
                _ => break,
            }
        }

        // Check for valid characters after the value.
        check_value_end(stream, cur_brace)?;

        let (mut val1, _, _) = val_deque.pop_front().unwrap();
        while !op_deque.is_empty() {
            let (val2, line2, col2) = val_deque.pop_front().unwrap();
            val1 = binary_op_on_values(
                stream,
                val1,
                val2,
                op_deque.pop_front().unwrap(),
                line2,
                col2,
            )?;
        }
        Ok(val1)
    } else {
        Ok(res)
    }
}

fn parse_unary_op(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut Globals,
    depth: usize,
    cur_brace: Option<char>,
    ch: char,
) -> ParseResult<Value> {
    let _ = stream.next();
    let line = stream.line();
    let col = stream.col();

    let res = match stream.peek() {
        Some(_) => {
            parse_value(
                &mut stream,
                obj,
                &mut globals,
                line,
                col,
                depth,
                cur_brace,
                false,
            )?
        }
        None => return parse_err(stream.file(), UnexpectedEnd(line)),
    };
    unary_op_on_value(stream, res, ch, line, col)
}

// Get the next numeric (either Int or Frac) in the character stream.
fn parse_numeric(stream: &mut CharStream, line: usize, col: usize) -> ParseResult<Value> {
    let mut s1 = String::new();
    let mut s2 = String::new();
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
            '.' | ',' => {
                if !dec {
                    dec = true;
                } else {
                    return parse_err(
                        stream.file(),
                        InvalidValueChar(ch, stream.line(), stream.col()),
                    );
                }
            }
            _ => {
                return parse_err(
                    stream.file(),
                    InvalidValueChar(ch, stream.line(), stream.col()),
                )
            }
        }

        let _ = stream.next();
    }

    if dec {
        // Parse a Frac from a number with a decimal.
        if s1.is_empty() && s2.is_empty() {
            return parse_err(stream.file(), InvalidNumeric(line, col));
        }

        let whole: BigInt = if s1.is_empty() {
            0u8.into()
        } else {
            s1.parse().map_err(ParseError::from)?
        };

        // Remove trailing zeros.
        let s2 = s2.trim_right_matches('0');

        let (decimal, dec_len): (BigInt, usize) = if s2.is_empty() {
            (0u8.into(), 1)
        } else {
            (s2.parse().map_err(ParseError::from)?, s2.len())
        };

        let f = frac_from_whole_and_dec(whole, decimal, dec_len);
        Ok(f.into())
    } else {
        // Parse an Int.
        if s1.is_empty() {
            return parse_err(stream.file(), InvalidNumeric(line, col));
        }

        let i: BigInt = s1.parse().map_err(ParseError::from)?;
        Ok(i.into())
    }
}

// Parse a variable name and get a value from the corresponding variable.
fn parse_variable(
    stream: &mut CharStream,
    obj: &Obj,
    globals: &Globals,
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
                return parse_err(
                    stream.file(),
                    InvalidValueChar(ch, stream.line(), stream.col()),
                )
            }
        }

        let _ = stream.next();
    }

    match var.as_str() {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        "null" => Ok(Value::Null),
        var @ "@" => parse_err(stream.file(), InvalidValue(var.into(), line, col)),
        var if is_global => {
            // Global variable, get value from globals map.
            match globals.get(var) {
                Some(value) => Ok(value.clone()),
                None => {
                    let var = String::from(var);
                    parse_err(stream.file(), GlobalNotFound(var, line, col))
                }
            }
        }
        var => {
            // Regular variable, get value from the current Obj.
            match obj.get(var) {
                Some(value) => Ok(value),
                None => {
                    let var = String::from(var);
                    parse_err(stream.file(), VariableNotFound(var, line, col))
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
            return parse_err(
                stream.file(),
                InvalidValueChar(ch, stream.line(), stream.col() - 1),
            )
        }
        Some(ch) => (false, ch),
        None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
    };

    if escape {
        ch = match stream.next() {
            Some(ch) => {
                match get_escape_char(ch) {
                    Some(ch) => ch,
                    None => {
                        return parse_err(
                            stream.file(),
                            InvalidEscapeChar(ch, stream.line(), stream.col() - 1),
                        )
                    }
                }
            }
            None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
        }
    }

    match stream.next() {
        Some('\'') => (),
        Some(ch) => {
            return parse_err(
                stream.file(),
                InvalidValueChar(ch, stream.line(), stream.col() - 1),
            )
        }
        None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
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
                    match get_escape_char(ch) {
                        Some(ch) => s.push(ch),
                        None => {
                            return parse_err(
                                stream.file(),
                                InvalidEscapeChar(ch, stream.line(), stream.col() - 1),
                            )
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
            None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
        }
    }

    Ok(s.into())
}

fn parse_include(
    mut stream: &mut CharStream,
    obj: &Obj,
    mut globals: &mut Globals,
    depth: usize,
) -> ParseResult<Value> {
    let ch = stream.next().unwrap();
    assert_eq!(ch, '<');

    // Go to the next non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
    }

    // Get the type token.
    let (tok_line, tok_col) = (stream.line(), stream.col());
    let token = parse_include_token(stream, tok_line, tok_col)?;

    // Go to the next non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
    }

    let (line, col) = (stream.line(), stream.col());
    let value = parse_value(
        &mut stream,
        obj,
        &mut globals,
        line,
        col,
        depth,
        Some('>'),
        true,
    )?;

    if value.get_type() != Type::Str {
        return parse_err(
            stream.file(),
            ExpectedType(value.get_type(), Type::Str, line, col),
        );
    }

    // Get the full path of the include file.
    let file2 = value.get_str().unwrap();
    let pathbuf = match stream.file().as_ref() {
        Some(file1) => Path::new(file1).parent().unwrap().join(Path::new(&file2)),
        None => Path::new(&file2).to_path_buf(),
    };
    let path = pathbuf.as_path();
    let path_str = match pathbuf.as_path().to_str() {
        Some(path) => path,
        None => return parse_err(stream.file(), InvalidIncludePath(file2, line, col)),
    };
    if !path.is_file() {
        return parse_err(stream.file(), InvalidIncludePath(file2, line, col));
    }
    let value: Value = match token.as_str() {
        "Obj" => parse_obj_file(path_str)?.into(),
        _ => return parse_err(stream.file(), InvalidIncludeToken(token, tok_line, tok_col)),
    };

    // Go to the next non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
    }

    match stream.next().unwrap() {
        '>' => (),
        ch => {
            return parse_err(
                stream.file(),
                InvalidClosingBracket(ch, Some('>'), stream.line(), stream.col() - 1),
            )
        }
    }

    Ok(value)
}

// Try to perform a unary operation on a single value.
fn unary_op_on_value(
    stream: &CharStream,
    val: Value,
    op: char,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    use types::Type::*;

    let t = val.get_type();

    Ok(match op {
        '+' => {
            match t {
                Int | Frac => val,
                _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
            }
        }
        '-' => {
            match t {
                Int => (-val.get_int().unwrap()).into(),
                Frac => (-val.get_frac().unwrap()).into(),
                _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
            }
        }
        _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
    })
}

// Try to perform an operation on two values.
fn binary_op_on_values(
    stream: &CharStream,
    mut val1: Value,
    mut val2: Value,
    op: char,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    use types::Type::*;

    let (mut type1, mut type2) = (val1.get_type(), val2.get_type());

    // If one value is an Int and the other is a Frac, promote the Int.
    if type1 == Int && type2 == Frac {
        val1 = Value::Frac(BigRational::new(val1.get_int().unwrap(), 1.into()));
        type1 = Frac;
    } else if type1 == Frac && type2 == Int {
        val2 = Value::Frac(BigRational::new(val2.get_int().unwrap(), 1.into()));
        type2 = Frac;
    }

    Ok(match op {
        '+' => {
            match type1 {
                Int if type2 == Int => (val1.get_int().unwrap() + val2.get_int().unwrap()).into(),
                Frac if type2 == Frac => {
                    (val1.get_frac().unwrap() + val2.get_frac().unwrap()).into()
                }
                Char if type2 == Char => {
                    let mut s = String::with_capacity(2);
                    s.push(val1.get_char().unwrap());
                    s.push(val2.get_char().unwrap());
                    s.into()
                }
                Char if type2 == Str => {
                    let str2 = val2.get_str().unwrap();
                    let mut s = String::with_capacity(1 + str2.len());
                    s.push(val1.get_char().unwrap());
                    s.push_str(&str2);
                    s.into()
                }
                Str if type2 == Char => {
                    let str1 = val1.get_str().unwrap();
                    let mut s = String::with_capacity(str1.len() + 1);
                    s.push_str(&str1);
                    s.push(val2.get_char().unwrap());
                    s.into()
                }
                Str if type2 == Str => {
                    let str1 = val1.get_str().unwrap();
                    let str2 = val2.get_str().unwrap();
                    let mut s = String::with_capacity(str1.len() + str2.len());
                    s.push_str(&str1);
                    s.push_str(&str2);
                    s.into()
                }
                Arr(_) if type1 == type2 => {
                    let (arr1, arr2) = (val1.get_arr().unwrap(), val2.get_arr().unwrap());
                    let mut new_arr = arr::Arr::from_vec(arr1.to_vec()).unwrap();

                    // Push each val from arr2 to new_arr.
                    // Because we know that the types are equal, we can safely unwrap below.
                    arr2.with_each(|val| new_arr.push(val.clone()).unwrap());

                    new_arr.into()
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type2, type1, op, line, col),
                    )
                }
            }
        }
        '-' => {
            match type1 {
                Int if type2 == Int => (val1.get_int().unwrap() - val2.get_int().unwrap()).into(),
                Frac if type2 == Frac => {
                    (val1.get_frac().unwrap() - val2.get_frac().unwrap()).into()
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type2, type1, op, line, col),
                    )
                }
            }
        }
        '*' => {
            match type1 {
                Int if type2 == Int => (val1.get_int().unwrap() * val2.get_int().unwrap()).into(),
                Frac if type2 == Frac => {
                    (val1.get_frac().unwrap() * val2.get_frac().unwrap()).into()
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type2, type1, op, line, col),
                    )
                }
            }
        }
        '/' => {
            match type1 {
                Int if type2 == Int => {
                    let (int1, int2) = (val1.get_int().unwrap(), val2.get_int().unwrap());
                    if int2.is_zero() {
                        return parse_err(stream.file(), InvalidNumeric(line, col));
                    }
                    BigRational::new(int1, int2).into()
                }
                Frac if type2 == Frac => {
                    let (frac1, frac2) = (val1.get_frac().unwrap(), val2.get_frac().unwrap());
                    if frac2.is_zero() {
                        return parse_err(stream.file(), InvalidNumeric(line, col));
                    }
                    (frac1 / frac2).into()
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type2, type1, op, line, col),
                    )
                }
            }
        }
        '%' => {
            match type1 {
                Int if type2 == Int => {
                    let int2 = val2.get_int().unwrap();
                    if int2.is_zero() {
                        return parse_err(stream.file(), InvalidNumeric(line, col));
                    }
                    (val1.get_int().unwrap() % int2).into()
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type2, type1, op, line, col),
                    )
                }
            }
        }
        _ => {
            return parse_err(
                stream.file(),
                BinaryOperatorError(type2, type1, op, line, col),
            )
        }
    })
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

// Returns all characters until the next whitespace.
// Returns false if we got to the end of the stream.
fn parse_include_token(stream: &mut CharStream, line: usize, col: usize) -> ParseResult<String> {
    let mut s = String::new();

    loop {
        match stream.peek() {
            Some(ch) => {
                if is_whitespace(ch) || is_end_delimiter(ch) {
                    break;
                }

                if !ch.is_alphabetic() {
                    return parse_err(
                        stream.file(),
                        InvalidIncludeChar(ch, stream.line(), stream.col()),
                    );
                }

                let _ = stream.next();
                s.push(ch);
            }
            None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
        }
    }

    match s.as_str() {
        "Obj" | "Str" | "Arr" | "Tup" => Ok(s),
        _ => parse_err(stream.file(), InvalidIncludeToken(s, line, col)),
    }
}

// Helper function to make sure values are followed by a correct end delimiter.
fn check_value_end(stream: &CharStream, cur_brace: Option<char>) -> ParseResult<()> {
    match stream.peek() {
        Some(ch) => {
            match ch {
                ch if is_value_end_char(ch) => {
                    if is_end_delimiter(ch) && Some(ch) != cur_brace {
                        parse_err(
                            stream.file(),
                            InvalidClosingBracket(ch, cur_brace, stream.line(), stream.col()),
                        )
                    } else {
                        Ok(())
                    }
                }
                ch => {
                    parse_err(
                        stream.file(),
                        InvalidValueChar(ch, stream.line(), stream.col()),
                    )
                }
            }
        }
        None => Ok(()),
    }
}
