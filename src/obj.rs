//! `Obj` module.
//! A hashmap of keys to values, where values can be any type, including other objects.

#![allow(missing_docs)]
#![allow(unused_imports)] // will complain about num_traits::Zero otherwise

use OverResult;
use arr::Arr;
use error::OverError;
use fraction::BigFraction;
use num::bigint::BigInt;
use num_traits::Zero;
use parse;
use parse::format::Format;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert;
use std::fmt;
use std::io;
use std::rc::Rc;
use std::str::FromStr;
use tup::Tup;
use types::Type;
use util::write_file_str;
use value::Value;

#[derive(Clone, Debug)]
struct ObjInner {
    fields: HashMap<String, Value>,
    parent: Option<Obj>,
}

/// `Obj` struct.
#[derive(Clone, Debug)]
pub struct Obj {
    inner: Rc<RefCell<ObjInner>>,
}

macro_rules! get_fn {
    ( $name:tt, $type:ty ) => {
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
    /// Returns a new empty `Obj`.
    pub fn new() -> Obj {
        Obj {
            inner: Rc::new(RefCell::new(ObjInner {
                fields: HashMap::new(),
                parent: None,
            })),
        }
    }

    /// Returns a new `Obj` loaded from a file.
    pub fn from_file(path: &str) -> OverResult<Obj> {
        parse::load_from_file(path).map_err(OverError::from)
    }

    /// Writes this `Obj` to given file in `.over` representation.
    ///
    /// # Notes
    /// Note that the fields of the `Obj` will be output in an unpredictable order.
    /// Also note that shorthand in the original file, including variables and file includes,
    /// is not preserved when parsing the file, and will not appear when writing to another file.
    pub fn write_to_file(&self, path: &str) -> OverResult<()> {
        write_file_str(path, &self.format(false, 0)).map_err(OverError::from)
    }

    /// Iterates over each `(String, Value)` pair in `self`, applying `Fn` `f`.
    pub fn with_each<F>(&self, mut f: F)
    where
        F: FnMut(&String, &Value),
    {
        for (field, value) in &self.inner.borrow().fields {
            f(field, value)
        }
    }

    /// Returns the number of fields for this `Obj` (children/parents not included).
    pub fn len(&self) -> usize {
        self.inner.borrow().fields.len()
    }

    /// Returns whether this `Obj` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.borrow().fields.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    /// Returns true iff the `Obj` contains `field`.
    pub fn contains(&self, field: &str) -> bool {
        self.inner.borrow().fields.contains_key(field)
    }

    /// Removes a field and its associated value from the `Obj`.
    pub fn remove(&mut self, field: &str) -> Option<Value> {
        match self.inner.borrow_mut().fields.remove(field) {
            Some(value) => Some(value),
            None => None,
        }
    }

    /// Clears all fields from the `Obj`.
    pub fn clear(&mut self) {
        self.inner.borrow_mut().fields.clear();
    }

    /// Gets the `Value` associated with `field`.
    pub fn get(&self, field: &str) -> Option<Value> {
        let inner = self.inner.borrow();

        match inner.fields.get(field) {
            Some(value) => Some(value.clone()),
            None => {
                match inner.parent {
                    Some(ref parent) => parent.get(field),
                    None => None,
                }
            }
        }
    }

    /// Gets the `Value` associated with `field` and the `Obj` where it was found (either `self` or
    /// one of its parents).
    pub fn get_with_source(&self, field: &str) -> Option<(Value, Obj)> {
        let inner = self.inner.borrow();

        match inner.fields.get(field) {
            Some(value) => Some((value.clone(), self.clone())),
            None => {
                match inner.parent {
                    Some(ref parent) => parent.get_with_source(field),
                    None => None,
                }
            }
        }
    }

    get_fn!(get_bool, bool);
    get_fn!(get_int, BigInt);
    get_fn!(get_frac, BigFraction);
    get_fn!(get_char, char);
    get_fn!(get_str, String);
    get_fn!(get_arr, Arr);
    get_fn!(get_tup, Tup);
    get_fn!(get_obj, Obj);

    /// Sets the `Value` for `field`.
    pub fn set(&mut self, field: &str, value: Value) {
        let _ = self.inner.borrow_mut().fields.insert(
            String::from(field),
            value,
        );
    }

    /// Returns whether this `Obj` has a parent.
    pub fn has_parent(&self) -> bool {
        self.inner.borrow().parent.is_some()
    }

    /// Returns the parent for this `Obj`.
    pub fn get_parent(&self) -> OverResult<Obj> {
        match self.inner.borrow().parent {
            Some(ref parent) => Ok(parent.clone()),
            None => Err(OverError::NoParentFound),
        }
    }

    /// Sets the parent for this `Obj`.
    /// Circular references in parents are not allowed.
    pub fn set_parent(&mut self, parent: &Obj) -> OverResult<()> {
        // Test for a circular reference.
        let mut cur_parent = parent.clone();
        if self.ptr_eq(&cur_parent) {
            return Err(OverError::CircularParentReferences);
        }
        while cur_parent.has_parent() {
            cur_parent = cur_parent.get_parent()?;
            if self.ptr_eq(&cur_parent) {
                return Err(OverError::CircularParentReferences);
            }
        }

        self.inner.borrow_mut().parent = Some(parent.clone());
        Ok(())
    }
}

impl Default for Obj {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, 0))
    }
}

impl FromStr for Obj {
    type Err = OverError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::load_from_str(s).map_err(OverError::from)
    }
}

impl PartialEq for Obj {
    fn eq(&self, other: &Self) -> bool {
        let inner = self.inner.borrow();
        let other_inner = other.inner.borrow();

        if inner.parent.is_some() && other_inner.parent.is_some() {
            let parent = self.get_parent().unwrap();
            let other_parent = other.get_parent().unwrap();
            if !parent.ptr_eq(&other_parent) {
                return false;
            }
        } else if !(inner.parent.is_none() && other_inner.parent.is_none()) {
            return false;
        }

        inner.fields == other_inner.fields
    }
}
