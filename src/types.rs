//! Module for types.

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
            Null => if let Null = *other { true } else { false },
            Bool => if let Bool = *other { true } else { false },
            Int => if let Int = *other { true } else { false },
            Frac => if let Frac = *other { true } else { false },
            Char => if let Char = *other { true } else { false },
            Str => if let Str = *other { true } else { false },
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
            Obj => if let Obj = *other { true } else { false },
        }
    }
}
impl Eq for Type {}
