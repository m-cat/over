//! An array container which can hold an arbitrary number of elements of a single type.

use crate::parse::format::Format;
use crate::types::Type;
use crate::value::Value;
use crate::{OverError, OverResult, INDENT_STEP};
use std::convert::TryFrom;
use std::fmt;
use std::slice::Iter;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct ArrInner {
    vec: Vec<Value>,
    inner_t: Type,
}

/// `Arr` struct.
#[derive(Clone, Debug)]
pub struct Arr {
    inner: Arc<ArrInner>,
}

impl Arr {
    /// Returns an empty `Arr`.
    pub fn empty() -> Self {
        Self::from_values_unchecked(vec![], Type::Any)
    }

    /// Returns a new `Arr` from the given vector of `Value`s.
    ///
    /// Checks that every value is of the same type.
    pub fn from_values(vec: Vec<Value>) -> OverResult<Self> {
        let mut tcur = Type::Any;
        let mut has_any = true;

        for value in &vec {
            let tnew = value.get_type();

            if has_any {
                match Type::most_specific(&tcur, &tnew) {
                    Some((t, any)) => {
                        tcur = t;
                        has_any = any;
                    }
                    None => return Err(OverError::ArrTypeMismatch(tcur, tnew)),
                }
            } else if tcur != tnew {
                return Err(OverError::ArrTypeMismatch(tcur, tnew));
            }
        }

        Ok(Self::from_values_unchecked(vec, tcur))
    }

    /// Returns a new `Arr` from the given vector of `Value`s without checking whether every value
    /// in `vec` is the same type.
    ///
    /// Requires the type so that we don't have to recalculate it (by going through every single
    /// value and calculating the most specific type).
    ///
    /// It is much faster than the safe version, [`from_values`], if you know every element in `vec`
    /// is of type `inner_t`.
    pub fn from_values_unchecked(vec: Vec<Value>, inner_t: Type) -> Self {
        Self {
            inner: Arc::new(ArrInner { vec, inner_t }),
        }
    }

    /// Returns a reference to the inner vec of this `Arr`.
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
            Err(OverError::ArrOutOfBounds(index))
        } else {
            Ok(self.inner.vec[index].clone())
        }
    }

    /// Returns the type of all elements in this `Arr`.
    pub fn inner_type(&self) -> Type {
        self.inner.inner_t.clone()
    }

    /// Returns the length of this `Arr`.
    pub fn len(&self) -> usize {
        self.inner.vec.len()
    }

    /// Returns whether this `Arr` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.vec.is_empty()
    }

    /// Returns whether `self` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    /// Returns an iterator over the Arr.
    pub fn iter(&self) -> Iter<Value> {
        self.vec_ref().iter()
    }
}

impl Default for Arr {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Arr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(true, INDENT_STEP))
    }
}

impl TryFrom<Vec<Value>> for Arr {
    type Error = OverError;

    fn try_from(vec: Vec<Value>) -> Result<Self, Self::Error> {
        Self::from_values(vec)
    }
}

impl PartialEq for Arr {
    fn eq(&self, other: &Self) -> bool {
        // Quickly return false if the types don't match.
        if self.inner.inner_t != other.inner.inner_t {
            return false;
        }

        self.inner.vec == other.inner.vec
    }
}

impl Eq for Arr {}
