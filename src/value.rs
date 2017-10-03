//! Module containing values and their inner values.

use fraction::Fraction;
use object::Object;

#[derive(Debug)]
pub enum Type {
    Bool,
    Int,
    Frac,
    Char,
    Str,
    Arr(Type),
    Tup(Type),
    Obj,
}

/// Enum of possible values.
#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Frac(Fraction),
    Char(char),
    Str(StringWrapper),
    Arr(ArrWrapper),
    Tup(TupWrapper),
    Obj(Object),
}

impl Value {
    pub fn get_bool(&self) -> Option<bool> {
        if let Value::Bool(inner) = *self {
            Some(inner)
        } else {
            None
        }
    }
}

/// Trait implemented by all inner values.
pub trait InnerValue: Sized {
    fn into_value(self) -> Value;
}

impl InnerValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl InnerValue for &'static str {
    fn into_value(self) -> Value {
        Value::Str(String::from(self))
    }
}

impl InnerValue for String {
    fn into_value(self) -> Value {
        Value::Str(self)
    }
}
