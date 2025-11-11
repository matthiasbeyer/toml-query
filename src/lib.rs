//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

#![recursion_limit = "1024"]
#![warn(rust_2018_idioms)]
// We need this for error_chain, unfortunately.

//! # toml-query
//!
//! A crate to help executing queries on toml data structures inside Rust code.

// external crates

#[macro_use]
extern crate is_match;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(all(test, feature = "typed"))]
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

// public modules

#[cfg(not(feature = "log"))]
#[macro_use]
pub mod log;

#[doc(hidden)]
pub use toml_query_derive::*;

pub mod delete;
pub mod error;
pub mod insert;
pub mod read;
pub mod set;
mod util;
pub mod value;

// private modules

mod resolver;
mod tokenizer;
