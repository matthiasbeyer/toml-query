/// The Toml Insert extensions

use toml::Value;

use tokenizer::Token;
use tokenizer::tokenize_with_seperator;
use error::*;

pub trait TomlValueInsertExt {

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
    /// If a Value is inserted into an Array, the array indexes are shifted. Semantically this is
    /// the same as doing a `array.insert(4, _)` (see the standard library).
    ///
    /// # Return value
    ///
    /// If the insert operation worked correctly, `Ok(None)` is returned.
    /// If the insert operation replaced an existing value `Ok(Some(old_value))` is returned
    /// On failure, `Err(e)` is returned
    ///
    fn insert_with_seperator(&mut self, query: &str, sep: char, value: Value) -> Result<Option<Value>>;

    /// Extension function for inserting a value from the current toml::Value document
    ///
    /// See documentation of `TomlValueinsertExt::insert_with_seperator`
    fn insert(&mut self, query: &str, value: Value) -> Result<Option<Value>> {
        self.insert_with_seperator(query, '.', value)
    }

}

impl TomlValueInsertExt for Value {

    fn insert_with_seperator(&mut self, query: &str, sep: char, value: Value) -> Result<Option<Value>> {
        use resolver::mut_resolver::resolve;

        let mut tokens = try!(tokenize_with_seperator(query, sep));
        let last       = tokens.pop_last().unwrap();
        let mut val    = try!(resolve(self, &tokens));

        match *last {
            Token::Identifier { ident, .. } => {
                match val {
                    &mut Value::Table(ref mut t) => {
                        Ok(t.insert(ident, value))
                    },
                    _ => Err(Error::from(ErrorKind::NoIdentifierInArray(ident.clone())))
                }
            },

            Token::Index { idx , .. } => {
                match val {
                    &mut Value::Array(ref mut a) => {
                        if a.len() > idx {
                            a.insert(idx, value);
                            Ok(None)
                        } else {
                            a.push(value);
                            Ok(None)
                        }
                    },
                    _ => Err(Error::from(ErrorKind::NoIndexInTable(idx)))
                }
            },
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use toml::Value;
    use toml::from_str as toml_from_str;

    #[test]
    fn test_insert_with_seperator_into_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("table.a"), '.', Value::Integer(1));

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());

        assert!(is_match!(toml, Value::Table(_)));
        match toml {
            Value::Table(ref t) => {
                assert!(!t.is_empty());

                let table = t.get("table");
                assert!(table.is_some());

                let table = table.unwrap();
                assert!(is_match!(table, &Value::Table(_)));
                match table {
                    &Value::Table(ref t) => {
                        assert!(!t.is_empty());

                        let a = t.get("a");
                        assert!(a.is_some());

                        let a = a.unwrap();
                        assert!(is_match!(a, &Value::Integer(1)));
                    },
                    _ => panic!("What just happenend?"),
                }
            },
            _ => panic!("What just happenend?"),
        }
    }

    #[test]
    fn test_insert_with_seperator_into_array() {
        use std::ops::Index;

        let mut toml : Value = toml_from_str(r#"
        array = []
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("array.[0]"), '.', Value::Integer(1));

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());

        assert!(is_match!(toml, Value::Table(_)));
        match toml {
            Value::Table(ref t) => {
                assert!(!t.is_empty());

                let array = t.get("array");
                assert!(array.is_some());

                let array = array.unwrap();
                assert!(is_match!(array, &Value::Array(_)));
                match array {
                    &Value::Array(ref a) => {
                        assert!(!a.is_empty());
                        assert!(is_match!(a.index(0), &Value::Integer(1)));
                    },
                    _ => panic!("What just happenend?"),
                }
            },
            _ => panic!("What just happenend?"),
        }
    }

    #[test]
    fn test_insert_with_seperator_into_nested_table() {
        let mut toml : Value = toml_from_str(r#"
        [a.b.c]
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("a.b.c.d"), '.', Value::Integer(1));

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());

        assert!(is_match!(toml, Value::Table(_)));
        match toml {
            Value::Table(ref outer) => {
                assert!(!outer.is_empty());
                let a_tab = outer.get("a");
                assert!(a_tab.is_some());

                let a_tab = a_tab.unwrap();
                assert!(is_match!(a_tab, &Value::Table(_)));
                match a_tab {
                    &Value::Table(ref a) => {
                        assert!(!a.is_empty());

                        let b_tab = a.get("b");
                        assert!(b_tab.is_some());

                        let b_tab = b_tab.unwrap();
                        assert!(is_match!(b_tab, &Value::Table(_)));
                        match b_tab {
                            &Value::Table(ref b) => {
                                assert!(!b.is_empty());

                                let c_tab = b.get("c");
                                assert!(c_tab.is_some());

                                let c_tab = c_tab.unwrap();
                                assert!(is_match!(c_tab, &Value::Table(_)));
                                match c_tab {
                                    &Value::Table(ref c) => {
                                        assert!(!c.is_empty());

                                        let d = c.get("d");
                                        assert!(d.is_some());

                                        let d = d.unwrap();
                                        assert!(is_match!(d, &Value::Integer(1)));
                                    },
                                    _ => panic!("What just happenend?"),
                                }
                            },
                            _ => panic!("What just happenend?"),
                        }
                    },
                    _ => panic!("What just happenend?"),
                }
            },
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_insert_with_seperator_into_table_where_array_is() {
        let mut toml : Value = toml_from_str(r#"
        table = []
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("table.a"), '.', Value::Integer(1));

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(is_match!(err.kind(), &ErrorKind::NoIdentifierInArray(_)));
    }

    #[test]
    fn test_insert_with_seperator_into_array_where_table_is() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("table.[0]"), '.', Value::Integer(1));

        assert!(res.is_err());

        let err = res.unwrap_err();
        assert!(is_match!(err.kind(), &ErrorKind::NoIndexInTable(_)));
    }

    #[test]
    fn test_insert_with_seperator_into_array_between_values() {
        use std::ops::Index;

        let mut toml : Value = toml_from_str(r#"
        array = [1, 2, 3, 4, 5]
        "#).unwrap();

        let res = toml.insert_with_seperator(&String::from("array.[2]"), '.', Value::Integer(6));

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());

        assert!(is_match!(toml, Value::Table(_)));
        match toml {
            Value::Table(ref t) => {
                assert!(!t.is_empty());

                let array = t.get("array");
                assert!(array.is_some());

                let array = array.unwrap();
                assert!(is_match!(array, &Value::Array(_)));
                match array {
                    &Value::Array(ref a) => {
                        assert!(!a.is_empty());
                        assert!(is_match!(a.index(0), &Value::Integer(1)));
                        assert!(is_match!(a.index(1), &Value::Integer(2)));
                        assert!(is_match!(a.index(2), &Value::Integer(6)));
                        assert!(is_match!(a.index(3), &Value::Integer(3)));
                        assert!(is_match!(a.index(4), &Value::Integer(4)));
                        assert!(is_match!(a.index(5), &Value::Integer(5)));
                    },
                    _ => panic!("What just happenend?"),
                }
            },
            _ => panic!("What just happenend?"),
        }
    }

}

