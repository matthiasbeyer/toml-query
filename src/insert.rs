/// The Toml Insert extensions

use toml::Value;

use tokenizer::tokenize_with_seperator;
use error::*;

pub trait TomlValueInsertExt<'doc> {

    /// Extension function for inserting a value in the current toml::Value document
    /// using a custom seperator.
    ///
    /// For difference to TomlSetExt::set() and friends, read [#semantics].
    ///
    /// # Semantics
    ///
    /// The function automatically creates intermediate data structures based on the query string.
    /// That means, if the query string is `"a.b.c.[0]"`, but only a table `"a"` exists in the
    /// document, the function automatically creates a table `"b"` inside `"a"` and `"c"` inside
    /// `"b"`, and an array in `"c"`. The array index is ignored if the array is created.
    ///
    /// If an Array exists, but the specified index is larger than the last index, the array will
    /// be expanded by one element: If the array has a length of 3, but the query string specifies
    /// that the element should be put at 1000, the function ignores the large index and simply
    /// appends the value to the index.
    ///
    /// # Return value
    ///
    /// If the insert operation worked correctly, `Ok(None)` is returned.
    /// If the insert operation replaced an existing value `Ok(Some(old_value))` is returned
    /// On failure, `Err(e)` is returned
    ///
    fn insert_with_seperator(&mut self, query: &String, sep: char, value: Value) -> Result<Option<Value>>;

    /// Extension function for inserting a value from the current toml::Value document
    ///
    /// See documentation of `TomlValueinsertExt::insert_with_seperator`
    fn insert(&mut self, query: &String, value: Value) -> Result<Option<Value>> {
        self.insert_with_seperator(query, '.', value)
    }

}

impl<'doc> TomlValueInsertExt<'doc> for Value {

    fn insert_with_seperator(&mut self, query: &String, sep: char, value: Value) -> Result<Option<Value>> {
        use resolver::mut_resolver::resolve;

        unimplemented!()
    }

}



