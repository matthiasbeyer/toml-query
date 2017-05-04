/// # toml-query
///
/// A crate to help executing queries on toml data structures inside Rust code.
///

// external crates

#[macro_use] extern crate error_chain;
#[macro_use] extern crate is_match;
extern crate toml;

#[cfg(test)]
#[macro_use] extern crate quickcheck;

// public modules

#[macro_use] pub mod log;
pub mod error;

// private modules

mod tokenizer;

