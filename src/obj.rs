//! `Obj` module.
//! A hashmap of keys to values, where values can be any type, including other objects.

#![allow(unused_imports)] // will complain about num_traits::Zero otherwise

use crate::arr::Arr;
use crate::error::OverError;
use crate::parse;
use crate::parse::format::Format;
use crate::tup::Tup;
use crate::types::Type;
use crate::util::{is_digit, write_file_str};
use crate::value::Value;
use crate::{OverResult, INDENT_STEP};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;
use std::collections::hash_map::{Iter, Keys, Values};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{convert, fmt, io};

lazy_static! {
    static ref CUR_ID: AtomicUsize = AtomicUsize::new(0);
}

fn get_id() -> usize {
    CUR_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug)]
struct ObjInner {
    map: HashMap<String, Value>,
    parent: Option<Obj>,
    id: usize,
}

/// `Obj` struct.
#[derive(Clone, Debug)]
pub struct Obj {
    inner: Arc<ObjInner>,
}

macro_rules! get_fn {
    ( $doc:expr, $name:tt, $type:ty ) => {
        #[doc=$doc]
        pub fn $name(&self, field: &str) -> OverResult<$type> {
            match self.get(field) {
                Some(value) => {
                    match value.$name() {
                        Ok(result) => Ok(result),
                        e @ Err(_) => e,
                    }
                }
                None => Err(OverError::FieldNotFound(field.into())),
            }
        }
    }
}

impl Obj {
    /// Returns a new `Obj` created from the given `HashMap`.
    ///
    /// Returns an error if the map contains an invalid field name.
    /// A valid field name must start with an alphabetic character or '_' and subsequent characters
    /// must be alphabetic, numeric, or '_'.
    pub fn from_map(obj_map: HashMap<String, Value>) -> OverResult<Obj> {
        for field in obj_map.keys() {
            if !Self::is_valid_field(field) {
                return Err(OverError::InvalidFieldName((*field).clone()));
            }
        }
        let id = get_id();

        Ok(Obj {
            inner: Arc::new(ObjInner {
                map: obj_map,
                parent: None,
                id,
            }),
        })
    }

    /// Returns a new `Obj` created from the given `HashMap` with given `parent`.
    ///
    /// Returns an error if the map contains an invalid field name.
    ///
    /// See `from_map` for more details.
    pub fn from_map_with_parent(obj_map: HashMap<String, Value>, parent: Obj) -> OverResult<Obj> {
        for field in obj_map.keys() {
            if !Self::is_valid_field(field) {
                return Err(OverError::InvalidFieldName(field.clone()));
            }
        }
        let id = get_id();

        Ok(Obj {
            inner: Arc::new(ObjInner {
                map: obj_map,
                parent: Some(parent),
                id,
            }),
        })
    }

    /// Returns a new `Obj` created from the given `HashMap`.
    ///
    /// It is faster than the safe version, `from_map`, if you know every field has a valid name.
    /// You can check ahead of time whether a field is valid with `is_valid_field`.
    ///
    /// See `from_map` for more details.
    pub fn from_map_unchecked(obj_map: HashMap<String, Value>) -> Obj {
        let id = get_id();

        Obj {
            inner: Arc::new(ObjInner {
                map: obj_map,
                parent: None,
                id,
            }),
        }
    }

    /// Returns a new `Obj` created from the given `HashMap` with given `parent`.
    ///
    /// It is faster than the safe version, `from_map_with_parent`, if you know every field has
    /// a valid name. You can check ahead of time whether a field is valid with `is_valid_field`.
    ///
    /// See `from_map` for more details.
    pub fn from_map_with_parent_unchecked(obj_map: HashMap<String, Value>, parent: Obj) -> Obj {
        let id = get_id();

        Obj {
            inner: Arc::new(ObjInner {
                map: obj_map,
                parent: Some(parent),
                id,
            }),
        }
    }

    /// Returns the ID of this `Obj`.
    ///
    /// Every `Obj` is assigned its own globally unique ID. IDs are generated incrementally,
    /// starting at 0 for the first `Obj` created.
    ///
    /// # Notes
    /// The ID is ignored when testing `Obj` equality.
    pub fn id(&self) -> usize {
        self.inner.id
    }

    /// Returns a reference to the inner map of this `Obj`.
    pub fn map_ref(&self) -> &HashMap<String, Value> {
        &self.inner.map
    }

    /// Returns a new `Obj` loaded from a file.
    pub fn from_file(path: &str) -> OverResult<Obj> {
        Ok(parse::load_from_file(path)?)
    }

    /// Writes this `Obj` to given file in `.over` representation.
    ///
    /// # Notes
    /// Note that the fields of the `Obj` will be output in an unpredictable order.
    /// Also note that shorthand in the original file, including variables and file includes,
    /// is not preserved when parsing the file, and will not appear when writing to another file.
    pub fn write_to_file(&self, path: &str) -> OverResult<()> {
        write_file_str(path, &self.write_str())?;
        Ok(())
    }

    /// Writes this `Obj` to a `String`.
    ///
    /// # Notes
    /// See `write_to_file`.
    pub fn write_str(&self) -> String {
        self.format(false, 0)
    }

    /// Iterates over each `(String, Value)` pair in `self`, applying `f`.
    pub fn with_each<F>(&self, mut f: F)
    where
        F: FnMut(&String, &Value),
    {
        for (field, value) in &self.inner.map {
            f(field, value)
        }
    }

    /// Returns the number of fields for this `Obj` (parent fields not included).
    pub fn len(&self) -> usize {
        self.inner.map.len()
    }

    /// Returns whether this `Obj` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.map.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// Returns true if this `Obj` contains `field`.
    pub fn contains(&self, field: &str) -> bool {
        self.inner.map.contains_key(field)
    }

    /// Gets the `Value` associated with `field`.
    pub fn get(&self, field: &str) -> Option<Value> {
        match self.inner.map.get(field) {
            Some(value) => Some(value.clone()),
            None => match self.inner.parent {
                Some(ref parent) => parent.get(field),
                None => None,
            },
        }
    }

    /// Gets the `Value` associated with `field` and the `Obj` where it was found (either `self` or
    /// one of its parents).
    pub fn get_with_source(&self, field: &str) -> Option<(Value, Obj)> {
        match self.inner.map.get(field) {
            Some(value) => Some((value.clone(), self.clone())),
            None => match self.inner.parent {
                Some(ref parent) => parent.get_with_source(field),
                None => None,
            },
        }
    }

    get_fn!(
        "Returns the `bool` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Bool`.",
        get_bool,
        bool
    );
    get_fn!(
        "Returns the `BigInt` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Int`.",
        get_int,
        BigInt
    );
    get_fn!(
        "Returns the `BigRational` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Frac`.",
        get_frac,
        BigRational
    );
    get_fn!(
        "Returns the `char` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Char`.",
        get_char,
        char
    );
    get_fn!(
        "Returns the `String` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Str`.",
        get_str,
        String
    );
    get_fn!(
        "Returns the `Arr` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Arr`.",
        get_arr,
        Arr
    );
    get_fn!(
        "Returns the `Tup` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Tup`.",
        get_tup,
        Tup
    );
    get_fn!(
        "Returns the `Obj` found at `field`. \
         Returns an error if the field was not found \
         or if the `Value` at `field` is not `Obj`.",
        get_obj,
        Obj
    );

    /// Returns whether this `Obj` has a parent.
    pub fn has_parent(&self) -> bool {
        self.inner.parent.is_some()
    }

    /// Returns the parent for this `Obj`.
    pub fn get_parent(&self) -> Option<Obj> {
        match self.inner.parent {
            Some(ref parent) => Some(parent.clone()),
            None => None,
        }
    }

    /// Returns true if `field` is a valid field name for an `Obj`.
    ///
    /// The first character must be alphabetic or '_'. Subsequent characters are allowed to be
    /// alphabetic, digits, or '_'.
    pub fn is_valid_field(field: &str) -> bool {
        let mut first = true;

        for ch in field.chars() {
            if first {
                if !Self::is_valid_field_char(ch, true) {
                    return false;
                }
                first = false;
            } else if !Self::is_valid_field_char(ch, false) {
                return false;
            }
        }

        true
    }

    /// Returns true if the given char is valid for a field, depending on whether it is the first
    /// char or not.
    ///
    /// See `is_valid_field` for more details.
    pub fn is_valid_field_char(ch: char, first: bool) -> bool {
        match ch {
            ch if ch.is_alphabetic() => true,
            ch if is_digit(ch) => !first,
            '_' => true,
            '^' => first,
            _ => false,
        }
    }

    /// An iterator visiting all fields (keys) in arbitrary order.
    pub fn keys(&self) -> Keys<String, Value> {
        self.map_ref().keys()
    }

    /// An iterator visiting all values in arbitrary order.
    pub fn values(&self) -> Values<String, Value> {
        self.map_ref().values()
    }

    /// An iterator visiting all field-value pairs in arbitrary order.
    pub fn iter(&self) -> Iter<String, Value> {
        self.map_ref().iter()
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self::from_map_unchecked(map! {})
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, INDENT_STEP))
    }
}

impl FromStr for Obj {
    type Err = OverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse::load_from_str(s)?)
    }
}

/// For two Objs to be equal, the following two checks must pass:
/// 1. If either Obj has a parent, then both must have parents and the parents must be equal.
/// 2. The two Objs must have all the same fields pointing to the same values.
impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        let inner = &self.inner;
        let other_inner = &other.inner;

        // Check parent equality.
        if inner.parent.is_some() && other_inner.parent.is_some() {
            let parent = self.get_parent().unwrap();
            let other_parent = other.get_parent().unwrap();
            if parent != other_parent {
                return false;
            }
        } else if !(inner.parent.is_none() && other_inner.parent.is_none()) {
            return false;
        }

        // Check HashMap equality.
        inner.map == other_inner.map
    }
}
