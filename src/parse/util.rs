//! Utility functions used by the parser.

use fraction::BigFraction;
use num::bigint::BigUint;
use num_traits::{FromPrimitive, pow};

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

pub fn frac_from_whole_and_dec(
    whole: BigUint,
    decimal: BigUint,
    dec_len: usize,
    neg: bool,
) -> BigFraction {
    let denom = pow(BigUint::from_u8(10).unwrap(), dec_len);
    frac_from_whole(whole, neg) +
        if neg {
            BigFraction::new_neg(decimal, denom)
        } else {
            BigFraction::new(decimal, denom)
        }
}

pub fn frac_from_whole(whole: BigUint, neg: bool) -> BigFraction {
    if neg {
        BigFraction::new_neg(whole, 1u8)
    } else {
        BigFraction::new(whole, 1u8)
    }
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
