//! OVER: the best data format.
//!
//! # Examples
//!
//!

#![deny(missing_docs)]

extern crate fraction;
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

#[cfg(test)]
mod tests;

pub use error::OverError;
pub use obj::*;

/// Result type for this crate.
pub type OverResult<T> = Result<T, OverError>;
