/// The Toml Set extensions

use toml::Value;

use tokenizer::tokenize_with_seperator;
use error::*;

pub trait TomlValueSetExt<'doc> {

    /// Extension function for setting a value in the current toml::Value document
    /// using a custom seperator
    ///
    /// # Semantics
    ///
    /// The function _never_ creates intermediate data structures (Tables or Arrays) in the
    /// document.
    ///
    /// # Return value
    ///
    /// * If the set operation worked correctly, `Ok(None)` is returned.
    /// * If the set operation replaced an existing value `Ok(Some(old_value))` is returned
    /// * On failure, `Err(e)` is returned:
    ///     * If the query is `"a.b.c"` but there is no table `"b"`: error
    ///     * If the query is `"a.b.[0]"` but "`b"` is not an array: error
    ///     * If the query is `"a.b.[3]"` but the array at "`b"` has no index `3`: error
    ///     * etc.
    ///
    fn set_with_seperator(&mut self, query: &String, sep: char, value: Value) -> Result<Option<Value>>;

    /// Extension function for setting a value from the current toml::Value document
    ///
    /// See documentation of `TomlValueSetExt::set_with_seperator`
    fn set(&mut self, query: &String, value: Value) -> Result<Option<Value>> {
        self.set_with_seperator(query, '.', value)
    }

}

impl<'doc> TomlValueSetExt<'doc> for Value {

    fn set_with_seperator(&mut self, query: &String, sep: char, value: Value) -> Result<Option<Value>> {
        use resolver::mut_resolver::resolve;


    }

}


