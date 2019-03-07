//! Module containing parsing functions.

#![allow(clippy::too_many_arguments)]

use super::char_stream::CharStream;
use super::error::ParseErrorKind::*;
use super::error::{parse_err, ParseError};
use super::util::*;
use super::{ParseResult, MAX_DEPTH};
use crate::arr::{self, Arr};
use crate::obj::Obj;
use crate::tup::Tup;
use crate::types::Type;
use crate::value::Value;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Deref;
use std::path::Path;

type ObjMap = HashMap<String, Value>;
type GlobalMap = HashMap<String, Value>;
type IncludedMap = (HashMap<String, Value>, HashSet<String>);

lazy_static! {
    static ref OBJ_SENTINEL: Obj = Obj::from_map_unchecked(HashMap::new());
    static ref STR_SENTINEL: Obj = Obj::from_map_unchecked(HashMap::new());
    static ref ARR_SENTINEL: Obj = Obj::from_map_unchecked(HashMap::new());
    static ref TUP_SENTINEL: Obj = Obj::from_map_unchecked(HashMap::new());
}

/// Parses given file as an `Obj`.
pub fn parse_obj_file(path: &str) -> ParseResult<Obj> {
    let stream = CharStream::from_file(path)?;
    parse_obj_stream(stream, &mut (HashMap::new(), HashSet::new()))
}

// Parses given file as an `Obj`, keeping track of already encountered includes.
fn parse_obj_file_includes(path: &str, included: &mut IncludedMap) -> ParseResult<Obj> {
    let stream = CharStream::from_file(path)?;
    parse_obj_stream(stream, included)
}

/// Parses given &str as an `Obj`.
pub fn parse_obj_str(contents: &str) -> ParseResult<Obj> {
    let contents = String::from(contents);
    let stream = CharStream::from_string(contents)?;
    parse_obj_stream(stream, &mut (HashMap::new(), HashSet::new()))
}

// Parses an Obj given a character stream.
#[inline]
fn parse_obj_stream(mut stream: CharStream, mut included: &mut IncludedMap) -> ParseResult<Obj> {
    let mut obj: ObjMap = HashMap::new();

    // Go to the first non-whitespace character, or return if there is none.
    if !find_char(stream.clone()) {
        return Ok(Obj::from_map_unchecked(obj));
    }

    let mut globals: GlobalMap = HashMap::new();
    let mut parent = None;

    // Parse all field/value pairs for this Obj.
    while parse_field_value_pair(
        &mut stream,
        &mut obj,
        &mut globals,
        &mut included,
        &mut parent,
        1,
        None,
    )? {}

    Ok(match parent {
        Some(parent) => Obj::from_map_with_parent_unchecked(obj, parent),
        None => Obj::from_map_unchecked(obj),
    })
}

// Parses a sub-Obj in a file. It *must* start with { and end with }.
fn parse_obj(
    mut stream: &mut CharStream,
    globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
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

    let mut obj: ObjMap = HashMap::new();
    let mut parent = None;

    // Parse field/value pairs.
    while parse_field_value_pair(
        &mut stream,
        &mut obj,
        globals,
        &mut included,
        &mut parent,
        depth,
        Some('}'),
    )? {}

    let obj = match parent {
        Some(parent) => Obj::from_map_with_parent_unchecked(obj, parent),
        None => Obj::from_map_unchecked(obj),
    };
    Ok(obj.into())
}

// Parses a field/value pair.
#[inline]
fn parse_field_value_pair(
    mut stream: &mut CharStream,
    obj: &mut ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
    parent: &mut Option<Obj>,
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
            InvalidClosingBracket(cur_brace, peek, stream.line(), stream.col()),
        );
    }

    // Get the field line/col.
    let (field_line, field_col) = (stream.line(), stream.col());

    // Parse field.
    let (field, is_global, is_parent) = parse_field(stream.clone(), field_line, field_col)?;

    if !is_global && !is_parent && obj.contains_key(&field) {
        return parse_err(stream.file(), DuplicateField(field, field_line, field_col));
    } else if is_parent && parent.is_some() {
        return parse_err(
            stream.file(),
            DuplicateField("^".into(), field_line, field_col),
        );
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
        &mut included,
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
        let par = value
            .get_obj()
            .map_err(|e| ParseError::from_over(&e, stream.file(), value_line, value_col))?;
        *parent = Some(par);
    } else {
        obj.insert(field, value);
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

// Parses an Arr given a file.
fn parse_arr_file(path: &str, mut included: &mut IncludedMap) -> ParseResult<Arr> {
    let mut stream = CharStream::from_file(path)?;

    let obj: ObjMap = HashMap::new();
    let mut globals: GlobalMap = HashMap::new();

    let mut vec = Vec::new();
    let mut tcur = Type::Any;
    let mut has_any = true;

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            break;
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            &obj,
            &mut globals,
            &mut included,
            value_line,
            value_col,
            1,
            None,
            true,
        )?;

        let tnew = value.get_type();

        if has_any {
            match Type::most_specific(&tcur, &tnew) {
                Some((t, any)) => {
                    tcur = t;
                    has_any = any;
                }
                None => {
                    return parse_err(
                        stream.file(),
                        ExpectedType(tcur, tnew, value_line, value_col),
                    );
                }
            }
        } else if tcur != tnew {
            return parse_err(
                stream.file(),
                ExpectedType(tcur, tnew, value_line, value_col),
            );
        }

        vec.push(value);
    }

    let arr = Arr::from_vec_unchecked(vec, tcur);

    Ok(arr)
}

// Parses a sub-Arr in a file. It *must* start with [ and end with ].
fn parse_arr(
    mut stream: &mut CharStream,
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
    depth: usize,
) -> ParseResult<Value> {
    // Check depth.
    if depth > MAX_DEPTH {
        return parse_err(stream.file(), MaxDepth(stream.line(), stream.col()));
    }

    // We must already be at a '['.
    let ch = stream.next().unwrap();
    assert_eq!(ch, '[');

    let mut vec = Vec::new();
    let mut tcur = Type::Any;
    let mut has_any = true;

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
                InvalidClosingBracket(Some(']'), peek, stream.line(), stream.col()),
            );
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            obj,
            &mut globals,
            &mut included,
            value_line,
            value_col,
            depth,
            Some(']'),
            true,
        )?;

        let tnew = value.get_type();

        if has_any {
            match Type::most_specific(&tcur, &tnew) {
                Some((t, any)) => {
                    tcur = t;
                    has_any = any;
                }
                None => {
                    return parse_err(
                        stream.file(),
                        ExpectedType(tcur, tnew, value_line, value_col),
                    );
                }
            }
        } else if tcur != tnew {
            return parse_err(
                stream.file(),
                ExpectedType(tcur, tnew, value_line, value_col),
            );
        }

        vec.push(value);
    }

    let arr = Arr::from_vec_unchecked(vec, tcur);

    Ok(arr.into())
}

// Parses a Tup given a file.
fn parse_tup_file(path: &str, mut included: &mut IncludedMap) -> ParseResult<Tup> {
    let mut stream = CharStream::from_file(path)?;

    let mut vec: Vec<Value> = Vec::new();
    let obj: ObjMap = HashMap::new();
    let mut globals: GlobalMap = HashMap::new();

    loop {
        // Go to the first non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            break;
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            &obj,
            &mut globals,
            &mut included,
            value_line,
            value_col,
            1,
            None,
            true,
        )?;

        vec.push(value);
    }

    Ok(vec.into())
}

// Parses a sub-Tup in a file. It *must* start with ( and end with ).
fn parse_tup(
    mut stream: &mut CharStream,
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
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
                InvalidClosingBracket(Some(')'), peek, stream.line(), stream.col()),
            );
        }

        // At a non-whitespace character, parse value.
        let (value_line, value_col) = (stream.line(), stream.col());
        let value = parse_value(
            &mut stream,
            obj,
            &mut globals,
            &mut included,
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

// Gets the next field in the char stream.
// Returns Option<(field_name, is_global, is_parent)>.
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
            ch if Obj::is_valid_field_char(ch, first) => field.push(ch),
            ch => {
                return parse_err(
                    stream.file(),
                    InvalidFieldChar(ch, stream.line(), stream.col() - 1),
                );
            }
        }

        first = false;
    }

    // Check for invalid field names.
    match field.as_str() {
        _field_str if is_reserved(_field_str) => {
            parse_err(stream.file(), InvalidFieldName(field.clone(), line, col))
        }
        "^" => Ok((field.clone(), false, true)),
        bad if bad.starts_with('^') => {
            parse_err(stream.file(), InvalidFieldName(field.clone(), line, col))
        }
        _ => Ok((field.clone(), is_global, false)),
    }
}

// Gets the next value in the char stream.
fn parse_value(
    mut stream: &mut CharStream,
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
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
        '{' => parse_obj(&mut stream, &mut globals, included, depth + 1)?,
        '[' => parse_arr(&mut stream, obj, &mut globals, included, depth + 1)?,
        '(' => parse_tup(&mut stream, obj, &mut globals, included, depth + 1)?,
        '@' => parse_variable(
            &mut stream,
            obj,
            globals,
            included,
            line,
            col,
            depth,
            cur_brace,
        )?,
        '<' => parse_include(&mut stream, obj, &mut globals, &mut included, depth + 1)?,
        ch @ '+' | ch @ '-' => {
            parse_unary_op(&mut stream, obj, globals, included, depth, cur_brace, ch)?
        }
        ch if is_numeric_char(ch) => parse_numeric(&mut stream, line, col)?,
        ch if Obj::is_valid_field_char(ch, true) => parse_variable(
            &mut stream,
            obj,
            globals,
            included,
            line,
            col,
            depth,
            cur_brace,
        )?,
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
                        &mut included,
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
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
    depth: usize,
    cur_brace: Option<char>,
    ch: char,
) -> ParseResult<Value> {
    let _ = stream.next();
    let line = stream.line();
    let col = stream.col();

    let res = match stream.peek() {
        Some(_) => parse_value(
            &mut stream,
            obj,
            &mut globals,
            &mut included,
            line,
            col,
            depth + 1,
            cur_brace,
            false,
        )?,
        None => return parse_err(stream.file(), UnexpectedEnd(line)),
    };
    unary_op_on_value(stream, res, ch, line, col)
}

// Gets the next numeric (either Int or Frac) in the character stream.
fn parse_numeric(stream: &mut CharStream, line: usize, col: usize) -> ParseResult<Value> {
    let mut s1 = String::new();
    let mut s2 = String::new();
    let mut dec = false;
    let mut under = false;

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
            '_' => {
                if !under {
                    under = true;
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
                );
            }
        }

        if ch != '_' {
            under = false;
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
            s1.parse()?
        };

        // Remove trailing zeros.
        let s2 = s2.trim_end_matches('0');

        let (decimal, dec_len): (BigInt, usize) = if s2.is_empty() {
            (0u8.into(), 1)
        } else {
            (s2.parse()?, s2.len())
        };

        let f = frac_from_whole_and_dec(whole, decimal, dec_len);
        Ok(f.into())
    } else {
        // Parse an Int.
        if s1.is_empty() {
            return parse_err(stream.file(), InvalidNumeric(line, col));
        }

        let i: BigInt = s1.parse()?;
        Ok(i.into())
    }
}

// Parses a variable name and gets a value from the corresponding variable.
fn parse_variable(
    mut stream: &mut CharStream,
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
    line: usize,
    col: usize,
    depth: usize,
    cur_brace: Option<char>,
) -> ParseResult<Value> {
    let mut var = String::new();
    let mut is_global = false;
    let mut dot = false;
    let mut dot_global = false;

    let ch = stream.peek().unwrap();
    if ch == '@' {
        let ch = stream.next().unwrap();
        is_global = true;
        var.push(ch);
    }

    while let Some(ch) = stream.peek() {
        match ch {
            '.' => {
                let _ = stream.next();
                match stream.peek() {
                    Some('@') => dot_global = true,
                    Some(ch) if Obj::is_valid_field_char(ch, true) || is_numeric_char(ch) => (),
                    Some(ch) => {
                        return parse_err(
                            stream.file(),
                            InvalidValueChar(ch, stream.line(), stream.col()),
                        );
                    }
                    None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
                }

                dot = true;
                break;
            }
            ch if is_value_end_char(ch) => break,
            ch if Obj::is_valid_field_char(ch, false) => {
                let _ = stream.next();
                var.push(ch);
            }
            ch => {
                return parse_err(
                    stream.file(),
                    InvalidValueChar(ch, stream.line(), stream.col()),
                );
            }
        }
    }

    let mut value = match var.as_str() {
        "null" => Value::Null,
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),

        "Obj" => Value::Obj(OBJ_SENTINEL.clone()),
        "Str" => Value::Obj(STR_SENTINEL.clone()),
        "Arr" => Value::Obj(ARR_SENTINEL.clone()),
        "Tup" => Value::Obj(TUP_SENTINEL.clone()),

        var @ "@" => return parse_err(stream.file(), InvalidValue(var.into(), line, col)),
        var if is_global => {
            // Global variable, get value from globals map.
            match globals.get(var) {
                Some(value) => value.clone(),
                None => {
                    let var = String::from(var);
                    return parse_err(stream.file(), GlobalNotFound(var, line, col));
                }
            }
        }
        var => {
            // Regular variable, get value from the current Obj.
            match obj.get(var) {
                Some(value) => value.clone(),
                None => {
                    let var = String::from(var);
                    return parse_err(stream.file(), VariableNotFound(var, line, col));
                }
            }
        }
    };

    if dot {
        value = match value {
            Value::Arr(arr) => {
                let (line, col) = (stream.line(), stream.col());
                let value = parse_value(
                    &mut stream,
                    obj,
                    &mut globals,
                    &mut included,
                    line,
                    col,
                    depth + 1,
                    cur_brace,
                    false,
                )?;

                match value {
                    Value::Int(int) => match int.to_usize() {
                        Some(index) => arr
                            .get(index)
                            .map_err(|e| ParseError::from_over(&e, stream.file(), line, col))?,
                        None => return parse_err(stream.file(), InvalidIndex(int, line, col)),
                    },
                    _ => {
                        return parse_err(
                            stream.file(),
                            ExpectedType(Type::Int, value.get_type(), line, col),
                        );
                    }
                }
            }
            Value::Tup(tup) => {
                let (line, col) = (stream.line(), stream.col());
                let value = parse_value(
                    &mut stream,
                    obj,
                    &mut globals,
                    &mut included,
                    line,
                    col,
                    depth + 1,
                    cur_brace,
                    false,
                )?;

                match value {
                    Value::Int(int) => match int.to_usize() {
                        Some(index) => tup
                            .get(index)
                            .map_err(|e| ParseError::from_over(&e, stream.file(), line, col))?,
                        None => return parse_err(stream.file(), InvalidIndex(int, line, col)),
                    },
                    _ => {
                        return parse_err(
                            stream.file(),
                            ExpectedType(Type::Int, value.get_type(), line, col),
                        );
                    }
                }
            }
            Value::Obj(obj) => {
                let (line, col) = (stream.line(), stream.col());

                if dot_global {
                    return parse_err(stream.file(), InvalidValueChar('@', line, col));
                }

                parse_variable(
                    &mut stream,
                    obj.map_ref(),
                    globals,
                    included,
                    line,
                    col,
                    depth + 1,
                    cur_brace,
                )?
            }
            _ => return parse_err(stream.file(), InvalidDot(value.get_type(), line, col)),
        }
    }

    Ok(value)
}

// Gets the next Char in the character stream.
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
            );
        }
        Some(ch) => (false, ch),
        None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
    };

    if escape {
        ch = match stream.next() {
            Some(ch) => match get_escape_char(ch) {
                Some(ch) => ch,
                None => {
                    return parse_err(
                        stream.file(),
                        InvalidEscapeChar(ch, stream.line(), stream.col() - 1),
                    );
                }
            },
            None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
        }
    }

    match stream.next() {
        Some('\'') => (),
        Some(ch) => {
            return parse_err(
                stream.file(),
                InvalidValueChar(ch, stream.line(), stream.col() - 1),
            );
        }
        None => return parse_err(stream.file(), UnexpectedEnd(stream.line())),
    }

    Ok(ch.into())
}

fn parse_str_file(path: &str) -> ParseResult<String> {
    // Replace \r\n line endings with \n for consistency in internal handling.
    let s = read_file_str(path)?.replace("\r\n", "\n");

    Ok(s)
}

// Gets the next Str in the character stream.
// Assumes the Str starts and ends with quotation marks and does not include them in the Str.
// '"', '\' and '$' must be escaped with '\'.
// Newlines can either be the string "\n" ('\' followed by 'n') or the newline character '\n'.
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
                            );
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

    // Replace \r\n line endings with \n for consistency in internal handling.
    let s = s.replace("\r\n", "\n");

    Ok(s.into())
}

fn parse_include(
    mut stream: &mut CharStream,
    obj: &ObjMap,
    mut globals: &mut GlobalMap,
    mut included: &mut IncludedMap,
    depth: usize,
) -> ParseResult<Value> {
    enum IncludeType {
        Obj,
        Str,
        Arr,
        Tup,
    }

    // Check depth.
    if depth > MAX_DEPTH {
        return parse_err(stream.file(), MaxDepth(stream.line(), stream.col()));
    }

    let ch = stream.next().unwrap();
    assert_eq!(ch, '<');

    // Go to the next non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
    }

    let (mut line, mut col) = (stream.line(), stream.col());
    let mut value = parse_value(
        &mut stream,
        obj,
        &mut globals,
        &mut included,
        line,
        col,
        depth,
        Some('>'),
        true,
    )?;

    let mut include_type = IncludeType::Obj; // Default include type if no token is present.
    let mut parse_again = true; // True if an include token was found.
    match value {
        Value::Obj(ref obj) if obj.ptr_eq(&OBJ_SENTINEL) => include_type = IncludeType::Obj,
        Value::Obj(ref obj) if obj.ptr_eq(&STR_SENTINEL) => include_type = IncludeType::Str,
        Value::Obj(ref obj) if obj.ptr_eq(&ARR_SENTINEL) => include_type = IncludeType::Arr,
        Value::Obj(ref obj) if obj.ptr_eq(&TUP_SENTINEL) => include_type = IncludeType::Tup,
        Value::Str(_) => parse_again = false,
        _ => {
            return parse_err(
                stream.file(),
                InvalidIncludeToken(value.get_type(), line, col),
            );
        }
    }

    if parse_again {
        // Go to the next non-whitespace character, or error if there is none.
        if !find_char(stream.clone()) {
            return parse_err(stream.file(), UnexpectedEnd(stream.line()));
        }

        line = stream.line();
        col = stream.col();
        value = parse_value(
            &mut stream,
            obj,
            &mut globals,
            &mut included,
            line,
            col,
            depth,
            Some('>'),
            true,
        )?;
    }

    // Go to the next non-whitespace character, or error if there is none.
    if !find_char(stream.clone()) {
        return parse_err(stream.file(), UnexpectedEnd(stream.line()));
    }

    match stream.next().unwrap() {
        '>' => (),
        ch => {
            return parse_err(
                stream.file(),
                InvalidClosingBracket(Some('>'), ch, stream.line(), stream.col() - 1),
            );
        }
    }

    // Get the full path of the include file.
    let include_file = match value {
        Value::Str(s) => s,
        _ => {
            return parse_err(
                stream.file(),
                ExpectedType(Type::Str, value.get_type(), line, col),
            );
        }
    };

    let pathbuf = match stream.file().as_ref() {
        Some(file) => Path::new(file)
            .parent()
            .unwrap()
            .join(Path::new(&include_file)),
        None => Path::new(&include_file).to_path_buf(),
    };
    let path = pathbuf.as_path();
    if !path.is_file() {
        return parse_err(stream.file(), InvalidIncludePath(include_file, line, col));
    }

    // Get the include file as a path relative to the current working directory.
    let path_str = match path.to_str() {
        Some(path) => path,
        None => return parse_err(stream.file(), InvalidIncludePath(include_file, line, col)),
    };

    // Get the include file as an absolute path.
    let path = match path.canonicalize() {
        Ok(path) => path,
        Err(_) => return parse_err(stream.file(), InvalidIncludePath(include_file, line, col)),
    };
    let full_path_str = match path.to_str() {
        Some(path) => path,
        None => return parse_err(stream.file(), InvalidIncludePath(include_file, line, col)),
    };

    // Prevent cyclic includes by temporarily storing the current file path.
    let storing = if let Some(file) = stream.file() {
        let full_file = String::from(Path::new(&file).canonicalize().unwrap().to_str().unwrap());
        included.1.insert(full_file.clone());
        Some(full_file)
    } else {
        None
    };
    if included.1.contains(full_path_str) {
        return parse_err(stream.file(), CyclicInclude(include_file, line, col));
    }

    // Get either the tracked value or parse it if it's our first time seeing the include.
    let value = if included.0.contains_key(full_path_str) {
        let value = &included.0[full_path_str];
        value.clone()
    } else {
        let value: Value = match include_type {
            IncludeType::Obj => parse_obj_file_includes(path_str, included)?.into(),
            IncludeType::Str => parse_str_file(path_str)?.into(),
            IncludeType::Arr => parse_arr_file(path_str, included)?.into(),
            IncludeType::Tup => parse_tup_file(path_str, included)?.into(),
        };
        // Use full path as included key.
        included.0.insert(full_path_str.into(), value.clone());
        value
    };

    // Remove the stored file path.
    if let Some(file) = storing {
        included.1.remove(&file);
    }

    Ok(value)
}

// Tries to perform a unary operation on a single value.
fn unary_op_on_value(
    stream: &CharStream,
    val: Value,
    op: char,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    use crate::types::Type::*;

    let t = val.get_type();

    Ok(match op {
        '+' => match t {
            Int | Frac => val,
            _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
        },
        '-' => match t {
            Int => (-val.get_int().unwrap()).into(),
            Frac => (-val.get_frac().unwrap()).into(),
            _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
        },
        _ => return parse_err(stream.file(), UnaryOperatorError(t, op, line, col)),
    })
}

// Tries to perform an operation on two values.
fn binary_op_on_values(
    stream: &CharStream,
    mut val1: Value,
    mut val2: Value,
    op: char,
    line: usize,
    col: usize,
) -> ParseResult<Value> {
    use crate::types::Type::*;

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
                Arr(_) => {
                    match Type::most_specific(&type1, &type2) {
                        Some((t, _)) => {
                            let (arr1, arr2) = (val1.get_arr().unwrap(), val2.get_arr().unwrap());
                            let (mut vec1, mut vec2) =
                                (arr1.vec_ref().clone(), arr2.vec_ref().clone());

                            let mut vec = Vec::with_capacity(vec1.len() + vec2.len());
                            vec.append(&mut vec1);
                            vec.append(&mut vec2);

                            // Get the inner type.
                            let arr = if let Arr(ref t) = t {
                                // Because we know the type already, we can safely use `_unchecked`.
                                arr::Arr::from_vec_unchecked(vec, t.deref().clone())
                            } else {
                                panic!("Logic error")
                            };

                            arr.into()
                        }
                        None => {
                            return parse_err(
                                stream.file(),
                                BinaryOperatorError(type1, type2, op, line, col),
                            );
                        }
                    }
                }
                _ => {
                    return parse_err(
                        stream.file(),
                        BinaryOperatorError(type1, type2, op, line, col),
                    );
                }
            }
        }
        '-' => match type1 {
            Int if type2 == Int => (val1.get_int().unwrap() - val2.get_int().unwrap()).into(),
            Frac if type2 == Frac => (val1.get_frac().unwrap() - val2.get_frac().unwrap()).into(),
            _ => {
                return parse_err(
                    stream.file(),
                    BinaryOperatorError(type1, type2, op, line, col),
                );
            }
        },
        '*' => match type1 {
            Int if type2 == Int => (val1.get_int().unwrap() * val2.get_int().unwrap()).into(),
            Frac if type2 == Frac => (val1.get_frac().unwrap() * val2.get_frac().unwrap()).into(),
            _ => {
                return parse_err(
                    stream.file(),
                    BinaryOperatorError(type1, type2, op, line, col),
                );
            }
        },
        '/' => match type1 {
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
                    BinaryOperatorError(type1, type2, op, line, col),
                );
            }
        },
        '%' => match type1 {
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
                    BinaryOperatorError(type1, type2, op, line, col),
                );
            }
        },
        _ => {
            return parse_err(
                stream.file(),
                BinaryOperatorError(type1, type2, op, line, col),
            );
        }
    })
}

// Finds the next non-whitespace character, ignoring comments, and update stream position.
// Returns true if such a character was found or false if we got to the end of the stream.
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

// Helper function to make sure values are followed by a correct end delimiter.
fn check_value_end(stream: &CharStream, cur_brace: Option<char>) -> ParseResult<()> {
    match stream.peek() {
        Some(ch) => match ch {
            ch if is_value_end_char(ch) => {
                if is_end_delimiter(ch) && Some(ch) != cur_brace {
                    parse_err(
                        stream.file(),
                        InvalidClosingBracket(cur_brace, ch, stream.line(), stream.col()),
                    )
                } else {
                    Ok(())
                }
            }
            ch => parse_err(
                stream.file(),
                InvalidValueChar(ch, stream.line(), stream.col()),
            ),
        },
        None => Ok(()),
    }
}
