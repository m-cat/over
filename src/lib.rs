//! OVER: the best data format.

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate num_bigint;
extern crate num_rational;
extern crate num_traits;

#[macro_use]
mod util;

#[macro_use]
pub mod macros;

pub mod arr;
pub mod error;
pub mod obj;
pub mod tup;
pub mod types;
pub mod value;

mod parse;

#[cfg(test)]
mod tests;

pub use crate::error::OverError;
pub use crate::obj::Obj;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Result type for this crate.
pub type OverResult<T> = Result<T, OverError>;

/// Trait containing required functions for all reference types (Arr, Tup, and Obj).
pub trait ReferenceType: PartialEq + Eq {
    /// Returns the ID of this reference type.
    ///
    /// Every unique instance of a reference type is assigned its own globally unique ID. IDs are
    /// generated incrementally, starting at 0 for the first instance created.
    ///
    /// # Notes
    ///
    /// The ID is ignored when testing equality using `eq` or `==`.
    fn id(&self) -> usize;

    /// The number of references that exist to this unique instance of a reference type (minimum of
    /// one).
    fn num_references(&self) -> usize;

    /// Returns whether `self` and `other` point to the same data.
    fn ptr_eq(&self, other: &Self) -> bool;
}

// Indent step in .over files.
const INDENT_STEP: usize = 4;

lazy_static! {
    static ref CUR_ID: AtomicUsize = AtomicUsize::new(0);
}

// Generate a new, unique ID for an Arr, Tup, or Obj.
fn gen_id() -> usize {
    CUR_ID.fetch_add(1, Ordering::Relaxed)
}
