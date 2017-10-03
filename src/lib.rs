//! OVER: Overman's Awesome Thing.
//!
//! # Examples
//!
//!

extern crate fraction;
extern crate num_traits;

pub mod error;
#[macro_use]
pub mod macros;
pub mod object;
pub mod parser;
pub mod value;
pub mod wrappers;

#[cfg(test)]
mod tests;

pub use object::*;
