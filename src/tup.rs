//! `Tup` module.
//! A tuple container which can hold elements of different types.

use {INDENT_STEP, OverError, OverResult};
use parse::format::Format;
use std::fmt;
use std::rc::Rc;
use types::Type;
use value::Value;

#[derive(Clone, Debug)]
struct TupInner {
    vec: Vec<Value>,
    inner_tvec: Vec<Type>,
}

/// `Tup` struct.
#[derive(Clone, Debug)]
pub struct Tup {
    inner: Rc<TupInner>,
}

impl Tup {
    /// Returns a new `Tup` from the given vector of `Value`s.
    pub fn from_vec(values: Vec<Value>) -> Tup {
        let tvec: Vec<Type> = values.iter().map(|val| val.get_type()).collect();

        Tup {
            inner: Rc::new(TupInner {
                vec: values,
                inner_tvec: tvec,
            }),
        }
    }

    /// Returns the vector of values in this `Tup`.
    pub fn to_vec(&self) -> Vec<Value> {
        self.inner.vec.clone()
    }

    /// Returns a reference to the inner vec of this `Tup`.
    pub fn vec_ref(&self) -> &Vec<Value> {
        &self.inner.vec
    }

    /// Iterates over each `Value` in `self`, applying `Fn` `f`.
    pub fn with_each<F>(&self, mut f: F)
    where
        F: FnMut(&Value),
    {
        for value in &self.inner.vec {
            f(value)
        }
    }

    /// Gets the value at `index`.
    /// Returns an error if `index` is out of bounds.
    pub fn get(&self, index: usize) -> OverResult<Value> {
        if index >= self.inner.vec.len() {
            Err(OverError::TupOutOfBounds(index))
        } else {
            Ok(self.inner.vec[index].clone())
        }
    }

    /// Returns the type vector of this `Tup`.
    pub fn inner_type_vec(&self) -> Vec<Type> {
        self.inner.inner_tvec.clone()
    }

    /// Returns the length of this `Tup`.
    pub fn len(&self) -> usize {
        self.inner.vec.len()
    }

    /// Returns whether this `Tup` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.vec.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Default for Tup {
    fn default() -> Self {
        Self::from_vec(vec![])
    }
}

impl fmt::Display for Tup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, INDENT_STEP))
    }
}

impl From<Vec<Value>> for Tup {
    fn from(vec: Vec<Value>) -> Self {
        Self::from_vec(vec)
    }
}

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        // Quickly return false if the types don't match.
        if self.inner.inner_tvec != other.inner.inner_tvec {
            return false;
        }

        self.inner.vec == other.inner.vec
    }
}
