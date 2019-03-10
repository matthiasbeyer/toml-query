#![recursion_limit = "1024"]
// We need this for error_chain, unfortunately.

/// # toml-query
///
/// A crate to help executing queries on toml data structures inside Rust code.
///

// external crates

#[macro_use] extern crate is_match;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure_derive;
extern crate failure;
extern crate regex;
extern crate toml;

#[cfg(feature = "log")]
#[macro_use] extern crate log;

#[cfg(feature = "typed")]
extern crate serde;

#[cfg(all(test, feature = "typed"))]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use] extern crate quickcheck;

// public modules

#[cfg(not(feature = "log"))]
#[macro_use] pub mod log;

extern crate toml_query_derive;

#[doc(hidden)]
pub use toml_query_derive::*;

pub mod error;
pub mod read;
pub mod set;
pub mod insert;
pub mod delete;
pub mod value;
mod util;

// private modules

mod tokenizer;
mod resolver;

