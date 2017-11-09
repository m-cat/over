//! `Arr` module.
//! An array container which can hold an arbitrary number of elements of a single type.

use {OverError, OverResult};
use parse::format::Format;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use types::Type;
use value::Value;

#[derive(Clone, Debug)]
struct ArrInner {
    vec: Vec<Value>,
    t: Type,
}

/// `Arr` struct.
#[derive(Clone, Debug)]
pub struct Arr {
    inner: Rc<RefCell<ArrInner>>,
}

impl Arr {
    /// Returns a new, empty `Arr`.
    pub fn new() -> Arr {
        Arr {
            inner: Rc::new(RefCell::new(ArrInner {
                vec: Vec::new(),
                t: Type::Any,
            })),
        }
    }

    /// Returns a new `Arr` with the given value vector as elements.
    pub fn from_vec(vec: Vec<Value>) -> OverResult<Arr> {
        let mut t = Type::Any;

        for value in &vec {
            let tnew = value.get_type();
            if let Type::Any = t {
                t = tnew.clone()
            } else if t != tnew {
                return Err(OverError::ArrTypeMismatch(tnew, t));
            }
        }

        Ok(Arr { inner: Rc::new(RefCell::new(ArrInner { vec, t })) })
    }

    /// Returns the vector of values in this `Arr`.
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

    /// Returns the type of all elements in this `Arr`.
    pub fn get_type(&self) -> Type {
        self.inner.borrow().t.clone()
    }

    /// Gets the value at `index`.
    /// Returns an error if `index` is out of bounds.
    pub fn get(&self, index: usize) -> OverResult<Value> {
        let inner = self.inner.borrow();

        if index >= inner.vec.len() {
            Err(OverError::ArrOutOfBounds(index))
        } else {
            Ok(inner.vec[index].clone())
        }
    }

    /// Returns the length of this `Arr`.
    // TODO: test this
    pub fn len(&self) -> usize {
        self.inner.borrow().vec.len()
    }

    /// Returns whether this `Arr` is empty.
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
            Err(OverError::ArrOutOfBounds(index))
        } else {
            inner.vec[index] = value;
            Ok(())
        }
    }

    /// Adds a `Value` to the `Arr`.
    /// Returns an error if the new `Value` is type-incompatible with the `Arr`.
    pub fn push(&mut self, value: Value) -> OverResult<()> {
        let mut inner = self.inner.borrow_mut();

        let inner_type = inner.t.clone();
        let val_type = value.get_type();

        if val_type != inner_type {
            Err(OverError::ArrTypeMismatch(val_type, inner_type))
        } else {
            // Update type of this `Arr`.
            if inner_type.is(&Type::Any) {
                inner.t = val_type;
            }

            inner.vec.push(value);

            Ok(())
        }
    }

    /// Inserts a `Value` into the `Arr` at the given index.
    /// Returns an error if the new `Value` is type-incompatible with the `Arr`
    /// or if the index is out of bounds.
    pub fn insert(&mut self, index: usize, value: Value) -> OverResult<()> {
        let mut inner = self.inner.borrow_mut();

        let inner_type = inner.t.clone();
        let val_type = value.get_type();

        if index > inner.vec.len() {
            return Err(OverError::ArrOutOfBounds(index));
        }
        if val_type != inner_type {
            Err(OverError::ArrTypeMismatch(val_type, inner_type))
        } else {
            // Update type of this `Arr`.
            if inner_type.is(&Type::Any) {
                inner.t = val_type;
            }

            inner.vec.insert(index, value);

            Ok(())
        }
    }

    /// Removes and returns a `Value` from the `Arr` at the given index.
    /// Sets the Arr type to Any if the new length is 0, otherwise the type is left unchanged.
    /// Returns an error if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> OverResult<Value> {
        let mut inner = self.inner.borrow_mut();

        if index > inner.vec.len() {
            return Err(OverError::ArrOutOfBounds(index));
        }

        let res = inner.vec.remove(index);

        if inner.vec.is_empty() {
            inner.t = Type::Any;
        }

        Ok(res)
    }
}

impl Default for Arr {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Arr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, 0))
    }
}

impl PartialEq for Arr {
    fn eq(&self, other: &Self) -> bool {
        let inner = self.inner.borrow();
        let other_inner = other.inner.borrow();

        if inner.t != other_inner.t {
            return false;
        }

        inner.vec == other_inner.vec
    }
}
