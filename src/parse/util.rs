//! Utility functions used by the parser.

use fraction::Fraction;

// If `ch` preceded by a backslash together form an escape character, then return this char.
// Otherwise, return None.
pub fn escape_char(ch: char) -> Option<char> {
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

// Returns true if this character signifies the legal end of a value.
pub fn is_value_end_char(ch: char) -> bool {
    is_whitespace(ch) || is_end_delimiter(ch)
}


// Returns true if the given char is valid for a field, given whether it is the first char or not.
// The first character must be alphabetic.
// Subsequent characters are allowed to be alphabetic, a digit, or '_'.
pub fn is_valid_field_char(ch: char, first: bool) -> bool {
    match ch {
        ch if ch.is_alphabetic() => true,
        ch if is_digit(ch) => !first,
        '_' => !first,
        '^' => first,
        _ => false,
    }
}

pub fn frac_from_whole_and_dec(whole: i64, decimal: u64, dec_len: usize) -> Fraction {
    let frac = Fraction::new(decimal, 10u8.pow(dec_len as u32));
    if whole < 0 {
        -frac + frac_from_whole(whole)
    } else {
        frac + frac_from_whole(whole)
    }
}

pub fn frac_from_whole(whole: i64) -> Fraction {
    if whole < 0 {
        Fraction::new_neg(whole.abs() as u64, 1u8)
    } else {
        Fraction::new(whole as u64, 1u8)
    }
}

pub fn negate(n: i64, neg: bool) -> i64 {
    n * if neg { -1 } else { 1 }
}

// Returns true if the character is either whitespace or '#' (start of a comment).
pub fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() || ch == '#'
}

pub fn is_end_delimiter(ch: char) -> bool {
    match ch {
        ')' | ']' | '}' => true,
        _ => false,
    }
}

pub fn is_numeric_char(ch: char) -> bool {
    match ch {
        _ch if is_digit(_ch) => true,
        '+' | '-' | '.' => true,
        _ => false,
    }
}

pub fn is_digit(ch: char) -> bool {
    match ch {
        '0'...'9' => true,
        _ => false,
    }
}
