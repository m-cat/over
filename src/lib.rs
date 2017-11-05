//! OVER: the best data format.

#![deny(missing_docs)]

extern crate fraction;
extern crate num;
extern crate num_traits;

#[macro_use]
pub mod macros;

pub mod arr;
pub mod error;
pub mod obj;
pub mod tup;
pub mod types;
pub mod value;

mod parse;
mod util;

#[cfg(test)]
mod tests;

pub use error::OverError;
pub use obj::Obj;

/// Result type for this crate.
pub type OverResult<T> = Result<T, OverError>;
