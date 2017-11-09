//! Utility functions used by the parser.

use fraction::BigFraction;
use num::bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, Signed, pow};

/// If `ch` preceded by a backslash together form an escape character, then return this char.
/// Otherwise, return None.
pub fn get_escape_char(ch: char) -> Option<char> {
    match ch {
        '\\' => Some('\\'),
        '"' => Some('"'),
        '\'' => Some('\''),
        '$' => Some('$'),
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        _ => None,
    }
}

/// Returns true if this character signifies the legal end of a value.
pub fn is_value_end_char(ch: char) -> bool {
    is_whitespace(ch) || is_end_delimiter(ch) || is_operator(ch)
}

/// Returns true if the given char is valid for a field, given whether it is the first char or not.
/// The first character must be alphabetic.
/// Subsequent characters are allowed to be alphabetic, a digit, or '_'.
pub fn is_valid_field_char(ch: char, first: bool) -> bool {
    match ch {
        ch if ch.is_alphabetic() => true,
        ch if is_digit(ch) => !first,
        '_' => !first,
        '^' => first,
        _ => false,
    }
}

/// Returns true if the character is either whitespace or '#' (start of a comment).
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
        '.' | ',' => true,
        _ => false,
    }
}

pub fn is_priority_operator(ch: char) -> bool {
    match ch {
        '*' | '/' | '%' => true,
        _ => false,
    }
}

pub fn is_operator(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '%' => true,
        _ => false,
    }
}

pub fn is_digit(ch: char) -> bool {
    match ch {
        '0'...'9' => true,
        _ => false,
    }
}

pub fn int_to_frac(int: &BigInt) -> BigFraction {
    if int.is_negative() {
        BigFraction::new_neg(int.abs().to_biguint().unwrap(), 1u8)
    } else {
        BigFraction::new(int.to_biguint().unwrap(), 1u8)
    }
}

pub fn two_ints_to_frac(int1: &BigInt, int2: &BigInt) -> BigFraction {
    let neg = int1.is_negative() ^ int2.is_negative();
    let int1 = int1.abs().to_biguint().unwrap();
    let int2 = int2.abs().to_biguint().unwrap();

    if neg {
        BigFraction::new_neg(int1, int2)
    } else {
        BigFraction::new(int1, int2)
    }
}

pub fn frac_from_whole_and_dec(whole: BigUint, decimal: BigUint, dec_len: usize) -> BigFraction {
    let denom = pow(BigUint::from_u8(10).unwrap(), dec_len);
    BigFraction::new(whole, 1u8) + BigFraction::new(decimal, denom)
}
