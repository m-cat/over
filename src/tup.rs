//! Tup module.
//! A tuple container which can hold elements of different types.

use std::cell::RefCell;
use std::rc::Rc;
use types::Type;
use value::Value;

// /// Given an array of type; element pairs, converts each element to values with the specified
// /// types.
// #[macro_export]
// macro_rules! tup_value_vec {

// }

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
    /// Creates and returns a new `Tup` from a vector of values.
    pub fn from_vec(values: Vec<Value>) -> Tup {
        let tvec: Vec<Type> = values.iter().map(|val| val.get_type()).collect();
        let vec = values;

        Tup { inner: Rc::new(RefCell::new(TupInner { vec, tvec })) }
    }

    /// Returns the type vector of this `Tup`.
    pub fn get_type(&self) -> Vec<Type> {
        self.inner.borrow().tvec.clone()
    }

    /// Returns the length of this `Tup`.
    // TODO: test this
    pub fn len(&self) -> usize {
        self.inner.borrow().vec.len()
    }
}

// impl Clone for Tup {
//     fn clone(&self) -> Tup {
//         Tup { inner: inner.clone(), tvec: tvec.clone() }
//     }
// }

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

// impl Index<&str> for Tup {
//     type Output = OverResult<Value>;

//     fn index(&self, index: &str) -> Self::Output {
//         match index {
//             self.get()
//         }
//     }
// }

// impl IndexMut<&str> for Tup {
//     fn index_mut<'a>(&mut self, index: Side) -> &'a mut Weight {
//         match index {
//         }
//     }
// }
