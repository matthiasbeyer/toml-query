/// Error types

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[cfg(feature = "typed")]
    #[fail(display = "{}", _0)]
    TomlSerialize(#[cause] ::toml::ser::Error),

    #[cfg(feature = "typed")]
    #[fail(display = "{}", _0)]
    TomlDeserialize(#[cause] ::toml::de::Error),

    // Errors for tokenizer

    #[fail(display = "Parsing the query '{}' failed", _0)]
    QueryParsingError(String),

    #[fail(display = "The query on the TOML is empty")]
    EmptyQueryError,

    #[fail(display = "The passed query has an empty identifier")]
    EmptyIdentifier,

    #[fail(display = "The passed query tries to access an array but does not specify the index")]
    ArrayAccessWithoutIndex,

    #[fail(display = "The passed query tries to access an array but does not specify a valid index")]
    ArrayAccessWithInvalidIndex,

    // Errors for Resolver

    #[fail(display = "The identfier '{}' is not present in the document", _0)]
    IdentifierNotFoundInDocument(String),

    #[fail(display = "Got an index query '[{}]' but have table", _0)]
    NoIndexInTable(usize),

    #[fail(display = "Got an identifier query '{}' but have array", _0)]
    NoIdentifierInArray(String),

    #[fail(display = "Got an identifier query '{}' but have value", _0)]
    QueryingValueAsTable(String),

    #[fail(display = "Got an index query '{}' but have value", _0)]
    QueryingValueAsArray(usize),

    #[fail(display = "Cannot delete table '{:?}' which is not empty", _0)]
    CannotDeleteNonEmptyTable(Option<String>),

    #[fail(display = "Cannot delete array '{:?}' which is not empty", _0)]
    CannotDeleteNonEmptyArray(Option<String>),

    #[fail(display = "Cannot access {} because expected {}", _0, _1)]
    CannotAccessBecauseTypeMismatch(&'static str, &'static str),

    #[fail(display = "Cannot delete in array at {}, array has length {}", _0, _1)]
    ArrayIndexOutOfBounds( usize, usize),

    #[fail(display = "Type Error. Requested {}, but got {}", _0, _1)]
    TypeError(&'static str, &'static str),

    #[fail(display = "Value at '{}' not there", _0)]
    NotAvailable(String),

}

