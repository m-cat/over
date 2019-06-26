//! An array container which can hold an arbitrary number of elements of a single type.

use crate::parse::format::Format;
use crate::types::Type;
use crate::value::Value;
use crate::{OverError, OverResult, ReferenceType, INDENT_STEP};
use std::convert::TryFrom;
use std::fmt;
use std::slice::Iter;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct ArrInner {
    // List of values.
    values: Vec<Value>,
    // Type of each contained value.
    inner_t: Type,
    // Unique ID.
    id: usize,
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
    pub fn from_values(values: Vec<Value>) -> OverResult<Self> {
        let mut tcur = Type::Any;
        let mut has_any = true;

        for value in &values {
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

        Ok(Self::from_values_unchecked(values, tcur))
    }

    /// Returns a new `Arr` from the given vector of `Value`s without checking whether every value
    /// in `vec` is the same type.
    ///
    /// Requires the type so that we don't have to recalculate it (by going through every single
    /// value and calculating the most specific type).
    ///
    /// It is much faster than the safe version, [`from_values`], if you know every element in `vec`
    /// is of type `inner_t`.
    pub fn from_values_unchecked(values: Vec<Value>, inner_t: Type) -> Self {
        Self {
            inner: Arc::new(ArrInner {
                values,
                inner_t,
                id: crate::gen_id(),
            }),
        }
    }

    /// Returns a reference to the inner vec of this `Arr`.
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
            Err(OverError::ArrOutOfBounds(index))
        } else {
            Ok(self.inner.values[index].clone())
        }
    }

    /// Returns the type of all elements in this `Arr`.
    pub fn inner_type(&self) -> Type {
        self.inner.inner_t.clone()
    }

    /// Returns the length of this `Arr`.
    pub fn len(&self) -> usize {
        self.inner.values.len()
    }

    /// Returns whether this `Arr` is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.values.is_empty()
    }

    /// Returns an iterator over the Arr.
    pub fn iter(&self) -> Iter<Value> {
        self.values_ref().iter()
    }
}

impl ReferenceType for Arr {
    fn id(&self) -> usize {
        self.inner.id
    }

    fn num_references(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    fn ptr_eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
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

        self.inner.values == other.inner.values
    }
}

impl Eq for Arr {}
