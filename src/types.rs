//! Module for types.

use std::fmt;
use std::fmt::{Debug, Display};

/// Enum of possible types for `Value`.
#[derive(Clone, Debug)]
pub enum Type {
    /// A type used to indicate an empty Arr.
    Empty,
    /// Null value.
    Null,

    /// A boolean type.
    Bool,
    /// A signed integer type.
    Int,
    /// A fractional type.
    Frac,
    /// A character type.
    Char,
    /// A string type.
    Str,

    /// An array type, containing the type of its sub-elements.
    Arr(Box<Type>),
    /// A tuple type, containing the types of its sub-elements.
    Tup(Vec<Type>),
    /// An object type.
    Obj,
}

impl Type {
    /// Returns true if this type is strictly the same as `other`.
    pub fn is(&self, other: &Type) -> bool {
        use self::Type::*;

        match *self {
            Empty => if let Empty = *other { true } else { false },
            Null => if let Null = *other { true } else { false },
            Bool => if let Bool = *other { true } else { false },
            Int => if let Int = *other { true } else { false },
            Frac => if let Frac = *other { true } else { false },
            Char => if let Char = *other { true } else { false },
            Str => if let Str = *other { true } else { false },
            Arr(ref box1) => {
                if let Arr(ref box2) = *other {
                    box1.is(box2)
                } else {
                    false
                }
            }
            Tup(ref tvec1) => {
                if let Tup(ref tvec2) = *other {
                    if tvec1.len() != tvec2.len() {
                        return false;
                    }
                    tvec1.iter().zip(tvec2.iter()).all(|(t1, t2)| t1.is(t2))
                } else {
                    false
                }
            }
            Obj => if let Obj = *other { true } else { false },
        }
    }
}

/// Two types are considered equal if one of them is Empty or they have the same variant.
// TODO: tests for this
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use self::Type::*;

        // If either is Empty, always return `true`.
        if let Empty = *other {
            return true;
        }

        match *self {
            Empty => true,
            Null => self.is(other),
            Bool => self.is(other),
            Int => self.is(other),
            Frac => self.is(other),
            Char => self.is(other),
            Str => self.is(other),
            Arr(ref box1) => {
                if let Arr(ref box2) = *other {
                    box1 == box2
                } else {
                    false
                }
            }
            Tup(ref tvec1) => {
                if let Tup(ref tvec2) = *other {
                    tvec1 == tvec2
                } else {
                    false
                }
            }
            Obj => self.is(other),
        }
    }
}
impl Eq for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Type::*;

        match *self {
            Empty => write!(f, "Empty"),
            Null => write!(f, "Null"),
            Bool => write!(f, "Bool"),
            Int => write!(f, "Int"),
            Frac => write!(f, "Frac"),
            Char => write!(f, "Char"),
            Str => write!(f, "Str"),
            Arr(ref boxxy) => write!(f, "Arr({})", boxxy),
            Tup(ref tvec) => {
                write!(
                    f,
                    "Tup([{}])",
                    match tvec.iter().nth(0) {
                        Some(t1) => {
                            tvec.iter().skip(1).fold(format!("{}", t1), |s, t| {
                                format!("{}, {}", s, t)
                            })
                        }
                        None => String::from(""),
                    }
                )
            }
            Obj => write!(f, "Obj"),
        }
    }
}
