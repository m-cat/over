//! Module for values.

use OverResult;
use arr;
use error::OverError;
use fraction::Fraction;
use obj;
use tup;
use types::Type;

/// Enum of possible values and their inner types.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A null value.
    Null,

    // Copy values.
    /// A boolean value.
    Bool(bool),
    /// A signed integer value.
    Int(i64),
    /// A fractional value.
    Frac(Fraction),
    /// A character value.
    Char(char),
    /// A string value.
    Str(String),

    // Reference values.
    /// An array value.
    Arr(arr::Arr),
    /// A tuple value.
    Tup(tup::Tup),
    /// An object value.
    Obj(obj::Obj),
}

impl Value {
    /// Returns a Null value.
    pub fn null() -> Value {
        Value::Null
    }

    /// Returns a new Bool value.
    pub fn new_bool(inner: bool) -> Value {
        Value::Bool(inner)
    }

    /// Returns a new Int value.
    pub fn new_int(inner: i64) -> Value {
        Value::Int(inner)
    }

    /// Returns a new Frac value.
    pub fn new_frac(inner: Fraction) -> Value {
        Value::Frac(inner)
    }

    /// Returns a new Char value.
    pub fn new_char(inner: char) -> Value {
        Value::Char(inner)
    }

    /// Returns a new Str value.
    pub fn new_str(inner: &str) -> Value {
        Value::Str(String::from(inner))
    }

    /// Returns a new Arr value.
    pub fn new_arr(inner: arr::Arr) -> Value {
        Value::Arr(inner)
    }

    /// Returns a new Tup value.
    pub fn new_tup(inner: tup::Tup) -> Value {
        Value::Tup(inner)
    }

    /// Returns a new Obj value.
    pub fn new_obj(inner: obj::Obj) -> Value {
        Value::Obj(inner)
    }

    /// Returns true iff this `Value` is null.
    pub fn is_null(&self) -> bool {
        if let Value::Null = *self { true } else { false }
    }

    /// Returns the `Type` of this `Value`.
    pub fn get_type(&self) -> Type {
        use self::Value::*;

        let res = match *self {
            Null => Type::Null,
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Frac(_) => Type::Frac,
            Char(_) => Type::Char,
            Str(_) => Type::Str,
            Arr(ref arr) => Type::Arr(Box::new(arr.get_type())),
            Tup(ref tup) => Type::Tup(tup.get_type()),
            Obj(_) => Type::Obj,
        };

        res
    }

    /// Returns the `bool` contained in this `Value`.
    /// Returns an error if this `Value` is not Bool.
    pub fn get_bool(&self) -> OverResult<bool> {
        if let Value::Bool(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `i64` contained in this `Value`.
    /// Returns an error if this `Value` is not Int.
    pub fn get_int(&self) -> OverResult<i64> {
        if let Value::Int(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `Fraction` contained in this `Value`.
    /// Returns an error if this `Value` is not Frac.
    pub fn get_frac(&self) -> OverResult<Fraction> {
        if let Value::Frac(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `char` contained in this `Value`.
    /// Returns an error if this `Value` is not Char.
    pub fn get_char(&self) -> OverResult<char> {
        if let Value::Char(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `String` contained in this `Value`.
    /// Returns an error if this `Value` is not Str.
    pub fn get_str(&self) -> OverResult<String> {
        if let Value::Str(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `Arr` contained in this `Value`.
    /// Returns an error if this `Value` is not Arr.
    pub fn get_arr(&self) -> OverResult<arr::Arr> {
        if let Value::Arr(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `Tup` contained in this `Value`.
    /// Returns an error if this `Value` is not Tup.
    pub fn get_tup(&self) -> OverResult<tup::Tup> {
        if let Value::Tup(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch)
        }
    }

    /// Returns the `Obj` contained in this `Value`.
    /// Returns an error if this `Value` is not Obj.
    pub fn get_obj(&self) -> OverResult<obj::Obj> {
        if let Value::Obj(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch)
        }
    }
}

macro_rules! impl_eq {
    ($valtype:ident, $value:ty) => {
        impl PartialEq<$value> for Value {
            fn eq(&self, other: &$value) -> bool {
                match *self {
                    Value::$valtype(ref value) => value == other,
                    _                         => false
                }
            }
        }

        impl PartialEq<Value> for $value {
            fn eq(&self, other: &Value) -> bool {
                match *other {
                    Value::$valtype(ref value) => value == self,
                    _ => false
                }
            }
        }
    }
}

impl_eq!(Bool, bool);
impl_eq!(Int, i64);
impl_eq!(Frac, Fraction);
impl_eq!(Char, char);
impl_eq!(Str, String);
impl_eq!(Arr, arr::Arr);
impl_eq!(Tup, tup::Tup);
impl_eq!(Obj, obj::Obj);

impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match *self {
            Value::Str(ref value) => value == other,
            _ => false,
        }
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        match *other {
            Value::Str(ref value) => value == self,
            _ => false,
        }
    }
}
