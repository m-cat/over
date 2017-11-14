//! `Tup` module.
//! A tuple container which can hold elements of different types.

use {INDENT_STEP, OverError, OverResult};
use parse::format::Format;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use types::Type;
use value::Value;

#[derive(Clone, Debug)]
struct TupInner {
    vec: Vec<Value>,
    tvec: Vec<Type>,
}

/// `Tup` struct.
#[derive(Clone, Debug)]
pub struct Tup {
    inner: Rc<RefCell<TupInner>>,
}

impl Tup {
    /// Creates a new, empty Tup.
    pub fn new() -> Tup {
        Tup {
            inner: Rc::new(RefCell::new(TupInner {
                vec: Vec::new(),
                tvec: Vec::new(),
            })),
        }
    }

    /// Creates and returns a new `Tup` from a vector of values.
    pub fn from_vec(values: Vec<Value>) -> Tup {
        let tvec: Vec<Type> = values.iter().map(|val| val.get_type()).collect();
        let vec = values;

        Tup { inner: Rc::new(RefCell::new(TupInner { vec, tvec })) }
    }

    /// Returns the vector of values in this `Tup`.
    pub fn to_vec(&self) -> Vec<Value> {
        self.inner.borrow().vec.clone()
    }

    /// Iterates over each `Value` in `self`, applying `Fn` `f`.
    pub fn with_each<F>(&self, mut f: F)
    where
        F: FnMut(&Value),
    {
        for value in &self.inner.borrow().vec {
            f(value)
        }
    }

    /// Returns the type vector of this `Tup`.
    pub fn get_type(&self) -> Vec<Type> {
        self.inner.borrow().tvec.clone()
    }

    /// Gets the value at `index`.
    /// Returns an error if `index` is out of bounds.
    pub fn get(&self, index: usize) -> OverResult<Value> {
        let inner = self.inner.borrow();

        if index >= inner.vec.len() {
            Err(OverError::TupOutOfBounds(index))
        } else {
            Ok(inner.vec[index].clone())
        }
    }

    /// Returns the length of this `Tup`.
    // TODO: test this
    pub fn len(&self) -> usize {
        self.inner.borrow().vec.len()
    }

    /// Returns whether this `Tup` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.borrow().vec.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    /// Sets the value at `index` to `value`.
    /// Returns an error if `index` is out of bounds.
    pub fn set(&mut self, index: usize, value: Value) -> OverResult<()> {
        let mut inner = self.inner.borrow_mut();

        if index >= inner.vec.len() {
            Err(OverError::TupOutOfBounds(index))
        } else {
            inner.vec[index] = value;
            Ok(())
        }
    }
}

impl Default for Tup {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Tup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, INDENT_STEP))
    }
}

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        let inner = self.inner.borrow();
        let other_inner = other.inner.borrow();

        if inner.tvec != other_inner.tvec {
            return false;
        }

        inner.vec == other_inner.vec
    }
}

impl From<Vec<Value>> for Tup {
    fn from(vec: Vec<Value>) -> Self {
        Self::from_vec(vec)
    }
}
