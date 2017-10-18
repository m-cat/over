//! `Arr` module.
//! An array container which can hold an arbitrary number of elements of a single type.

use {OverError, OverResult};
use std::cell::RefCell;
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
                t: Type::Empty,
            })),
        }
    }

    /// Returns a new `Arr` with the given value vector as elements.
    pub fn from_vec(vec: Vec<Value>) -> OverResult<Arr> {
        let mut t = Type::Empty;

        for value in &vec {
            let tnew = value.get_type();
            if let Type::Empty = t {
                t = tnew.clone()
            } else if t != tnew {
                return Err(OverError::ArrTypeMismatch);
            }
        }

        Ok(Arr { inner: Rc::new(RefCell::new(ArrInner { vec, t })) })
    }

    /// Returns the type of all elements in this `Arr`.
    pub fn get_type(&self) -> Type {
        self.inner.borrow().t.clone()
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

    /// Returns whether this `Arr` and `other` point to the same data.
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    /// Adds a `Value` to the `Arr`.
    /// Returns an error if the new `Value` is type-incompatible with the `Arr`.
    pub fn add(&mut self, value: Value) -> OverResult<()> {
        // Should be impossible to add an "Empty" value.
        debug_assert_ne!(value.get_type(), Type::Empty);

        let mut inner = self.inner.borrow_mut();

        let val_type = value.get_type();
        if val_type != inner.t {
            Err(OverError::ArrTypeMismatch)
        } else {
            // Update type of this `Arr`.
            if inner.t == Type::Empty {
                inner.t = val_type;
            }

            inner.vec.push(value);

            Ok(())
        }
    }

    /// Inserts a `Value` into the `Arr` at the given index.
    /// Returns an error if the new `Value` is type-incompatible with the `Arr`.
    // TODO: finish this, copy from `add` above
    pub fn insert(&mut self, index: usize, value: Value) -> OverResult<()> {
        let mut inner = self.inner.borrow_mut();

        if value.get_type() != inner.t {
            Err(OverError::ArrTypeMismatch)
        } else {
            inner.vec.insert(index, value);
            Ok(())
        }
    }

    /// Removes and returns a `Value` from the `Arr` at the given index.
    /// Returns an error if the index is out of bounds.
    // TODO: implement this
    pub fn remove() -> OverResult<Value> {
        unimplemented!()
    }
}

impl Default for Arr {
    fn default() -> Self {
        Self::new()
    }
}

// impl Clone for Arr {
//     fn clone(&self) -> Arr {
//         Arr { inner: inner.clone(), t: t.clone() }
//     }
// }

impl PartialEq for Arr {
    fn eq(&self, other: &Self) -> bool {
        let inner = self.inner.borrow();
        let other_inner = other.inner.borrow();

        if inner.vec.len() != other_inner.vec.len() {
            return false;
        }
        if inner.t != other_inner.t {
            return false;
        }

        inner.vec == other_inner.vec
    }
}

// impl Index<&str> for Arr {
//     type Output = OverResult<Value>;

//     fn index(&self, index: &str) -> Self::Output {
//         match index {
//             self.get()
//         }
//     }
// }

// impl IndexMut<&str> for Arr {
//     fn index_mut<'a>(&mut self, index: Side) -> &'a mut Weight {
//         match index {
//         }
//     }
// }
