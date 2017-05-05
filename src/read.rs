/// The Toml Read extensions

use toml::Value;

use tokenizer::tokenize_with_seperator;
use error::*;

pub trait TomlValueReadExt<'doc> {

    /// Extension function for reading a value from the current toml::Value document
    /// using a custom seperator
    fn read_with_seperator(&'doc self, query: &String, sep: char) -> Result<&'doc Value>;

    /// Extension function for reading a value from the current toml::Value document mutably
    /// using a custom seperator
    fn read_mut_with_seperator(&'doc mut self, query: &String, sep: char) -> Result<&'doc mut Value>;

    /// Extension function for reading a value from the current toml::Value document
    fn read(&'doc self, query: &String) -> Result<&'doc Value> {
        self.read_with_seperator(query, '.')
    }

    /// Extension function for reading a value from the current toml::Value document mutably
    fn read_mut(&'doc mut self, query: &String) -> Result<&'doc mut Value> {
        self.read_mut_with_seperator(query, '.')
    }

}

impl<'doc> TomlValueReadExt<'doc> for Value {

    fn read_with_seperator(&'doc self, query: &String, sep: char) -> Result<&'doc Value> {
        use resolver::non_mut_resolver::resolve;

        tokenize_with_seperator(query, sep).and_then(move |tokens| resolve(self, &tokens))
    }

    fn read_mut_with_seperator(&'doc mut self, query: &String, sep: char) -> Result<&'doc mut Value> {
        use resolver::mut_resolver::resolve;

        tokenize_with_seperator(query, sep).and_then(move |tokens| resolve(self, &tokens))
    }

}

