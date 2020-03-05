//! Error types

use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "typed")]
    #[error("{}", _0)]
    TomlSerialize(#[from] ::toml::ser::Error),

    #[cfg(feature = "typed")]
    #[error("{}", _0)]
    TomlDeserialize(#[from] ::toml::de::Error),

    // Errors for tokenizer
    #[error("Parsing the query '{0}' failed")]
    QueryParsingError(String),

    #[error("The query on the TOML is empty")]
    EmptyQueryError,

    #[error("The passed query has an empty identifier")]
    EmptyIdentifier,

    #[error("The passed query tries to access an array but does not specify the index")]
    ArrayAccessWithoutIndex,

    #[error("The passed query tries to access an array but does not specify a valid index")]
    ArrayAccessWithInvalidIndex,

    // Errors for Resolver
    #[error("The identfier '{0}' is not present in the document")]
    IdentifierNotFoundInDocument(String),

    #[error("Got an index query '[{0}]' but have table")]
    NoIndexInTable(usize),

    #[error("Got an identifier query '{0}' but have array")]
    NoIdentifierInArray(String),

    #[error("Got an identifier query '{0}' but have value")]
    QueryingValueAsTable(String),

    #[error("Got an index query '{0}' but have value")]
    QueryingValueAsArray(usize),

    #[error("Cannot delete table '{0:?}' which is not empty")]
    CannotDeleteNonEmptyTable(Option<String>),

    #[error("Cannot delete array '{0:?}' which is not empty")]
    CannotDeleteNonEmptyArray(Option<String>),

    #[error("Cannot access {0} because expected {1}")]
    CannotAccessBecauseTypeMismatch(&'static str, &'static str),

    #[error("Cannot delete in array at {0}, array has length {1}")]
    ArrayIndexOutOfBounds(usize, usize),

    #[error("Cannot access array at {0}, array has length {1}")]
    IndexOutOfBounds(usize, usize),

    #[error("Type Error. Requested {0}, but got {1}")]
    TypeError(&'static str, &'static str),

    #[error("Value at '{0}' not there")]
    NotAvailable(String),
}
