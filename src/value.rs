//! Module for values.

use OverResult;
use arr;
use error::OverError;
use fraction::BigFraction;
use num::bigint::BigInt;
use obj;
use parse::format::Format;
use std::fmt;
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
    Int(BigInt),
    /// A fractional value.
    Frac(BigFraction),
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
    /// Returns true iff this `Value` is null.
    pub fn is_null(&self) -> bool {
        if let Value::Null = *self { true } else { false }
    }

    /// Returns the `Type` of this `Value`.
    pub fn get_type(&self) -> Type {
        use self::Value::*;

        match *self {
            Null => Type::Null,
            Bool(_) => Type::Bool,
            Int(_) => Type::Int,
            Frac(_) => Type::Frac,
            Char(_) => Type::Char,
            Str(_) => Type::Str,
            Arr(ref arr) => Type::Arr(Box::new(arr.get_type())),
            Tup(ref tup) => Type::Tup(tup.get_type()),
            Obj(_) => Type::Obj,
        }
    }

    /// Returns the `bool` contained in this `Value`.
    /// Returns an error if this `Value` is not `Bool`.
    pub fn get_bool(&self) -> OverResult<bool> {
        if let Value::Bool(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Bool))
        }
    }

    /// Returns the `BigInt` contained in this `Value`.
    /// Returns an error if this `Value` is not `Int`.
    pub fn get_int(&self) -> OverResult<BigInt> {
        if let Value::Int(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Int))
        }
    }

    /// Returns the `BigFraction` contained in this `Value`.
    /// Returns an error if this `Value` is not `Frac`.
    pub fn get_frac(&self) -> OverResult<BigFraction> {
        if let Value::Frac(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Frac))
        }
    }

    /// Returns the `char` contained in this `Value`.
    /// Returns an error if this `Value` is not `Char`.
    pub fn get_char(&self) -> OverResult<char> {
        if let Value::Char(inner) = *self {
            Ok(inner)
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Char))
        }
    }

    /// Returns the `String` contained in this `Value`.
    /// Returns an error if this `Value` is not `Str`.
    pub fn get_str(&self) -> OverResult<String> {
        if let Value::Str(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Str))
        }
    }

    /// Returns the `Arr` contained in this `Value`.
    /// Returns an error if this `Value` is not `Arr`.
    pub fn get_arr(&self) -> OverResult<arr::Arr> {
        if let Value::Arr(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(
                self.get_type(),
                Type::Arr(Box::new(Type::Any)),
            ))
        }
    }

    /// Returns the `Tup` contained in this `Value`.
    /// Returns an error if this `Value` is not `Tup`.
    pub fn get_tup(&self) -> OverResult<tup::Tup> {
        if let Value::Tup(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Tup(vec![])))
        }
    }

    /// Returns the `Obj` contained in this `Value`.
    /// Returns an error if this `Value` is not `Obj`.
    pub fn get_obj(&self) -> OverResult<obj::Obj> {
        if let Value::Obj(ref inner) = *self {
            Ok(inner.clone())
        } else {
            Err(OverError::TypeMismatch(self.get_type(), Type::Obj))
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, 0))
    }
}

// impl PartialEq

macro_rules! impl_eq {
    ($valtype:ident, $type:ty) => {
        impl PartialEq<$type> for Value {
            fn eq(&self, other: &$type) -> bool {
                match *self {
                    Value::$valtype(ref value) => value == other,
                    _                         => false
                }
            }
        }

        impl PartialEq<Value> for $type {
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
impl_eq!(Int, BigInt);
impl_eq!(Frac, BigFraction);
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

// impl From

macro_rules! impl_from {
    ( $type:ty, $fn:tt ) => {
        impl From<$type> for Value {
            fn from(inner: $type) -> Self {
                Value::$fn(inner.into())
            }
        }
    };
}

impl_from!(bool, Bool);

impl_from!(usize, Int);
impl_from!(u8, Int);
impl_from!(u16, Int);
impl_from!(u32, Int);
impl_from!(u64, Int);
impl_from!(i8, Int);
impl_from!(i16, Int);
impl_from!(i32, Int);
impl_from!(i64, Int);
impl_from!(BigInt, Int);

impl_from!(f32, Frac);
impl_from!(f64, Frac);
impl_from!(BigFraction, Frac);

impl_from!(char, Char);

impl_from!(String, Str);
impl<'a> From<&'a str> for Value {
    fn from(inner: &str) -> Self {
        Value::Str(inner.into())
    }
}

impl_from!(arr::Arr, Arr);

impl_from!(tup::Tup, Tup);

impl_from!(obj::Obj, Obj);
