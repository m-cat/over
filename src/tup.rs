//! A tuple container which can hold elements of different types.

use crate::parse::format::Format;
use crate::types::Type;
use crate::value::Value;
use crate::{OverError, OverResult, INDENT_STEP};
use std::fmt;
use std::slice::Iter;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct TupInner {
    // List of values.
    values: Vec<Value>,
    // List of types for each contained value.
    inner_tvec: Vec<Type>,
    // Unique ID.
    id: usize,
}

/// `Tup` struct.
#[derive(Clone, Debug)]
pub struct Tup {
    inner: Arc<TupInner>,
}

impl Tup {
    /// Returns an empty `Tup`.
    pub fn empty() -> Self {
        Self::from_values(vec![])
    }

    /// Returns a new `Tup` from the given vector of `Value`s.
    pub fn from_values(values: Vec<Value>) -> Self {
        let inner_tvec: Vec<Type> = values.iter().map(|val| val.get_type()).collect();

        Self {
            inner: Arc::new(TupInner {
                values,
                inner_tvec,
                id: crate::gen_id(),
            }),
        }
    }

    /// Returns a reference to the inner vec of this `Tup`.
    pub fn values_ref(&self) -> &Vec<Value> {
        &self.inner.values
    }

    /// Iterates over each `Value` in `self`, applying `Fn` `f`.
    pub fn with_each<F>(&self, mut f: F)
    where
        F: FnMut(&Value),
    {
        for value in &self.inner.values {
            f(value)
        }
    }

    /// Gets the value at `index`.
    /// Returns an error if `index` is out of bounds.
    pub fn get(&self, index: usize) -> OverResult<Value> {
        if index >= self.inner.values.len() {
            Err(OverError::TupOutOfBounds(index))
        } else {
            Ok(self.inner.values[index].clone())
        }
    }

    /// Returns the type vector of this `Tup`.
    pub fn inner_type_vec(&self) -> Vec<Type> {
        self.inner.inner_tvec.clone()
    }

    /// Returns the length of this `Tup`.
    pub fn len(&self) -> usize {
        self.inner.values.len()
    }

    /// Returns whether this `Tup` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.values.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// Returns an iterator over the Tup.
    pub fn iter(&self) -> Iter<Value> {
        self.values_ref().iter()
    }
}

impl Default for Tup {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Tup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, INDENT_STEP))
    }
}

impl From<Vec<Value>> for Tup {
    fn from(vec: Vec<Value>) -> Self {
        Self::from_values(vec)
    }
}

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        // Quickly return false if the types don't match.
        if self.inner.inner_tvec != other.inner.inner_tvec {
            return false;
        }

        self.inner.values == other.inner.values
    }
}

impl Eq for Tup {}
