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

/// Result type for this crate.
pub type OverResult<T> = Result<T, OverError>;

// Indent step in .over files.
const INDENT_STEP: usize = 4;
