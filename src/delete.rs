/// The Toml Delete extensions

use toml::Value;

use tokenizer::Token;
use tokenizer::tokenize_with_seperator;
use error::*;

pub trait TomlValueDeleteExt {

    /// Extension function for deleting a value in the current toml::Value document
    /// using a custom seperator.
    ///
    /// # Semantics
    ///
    /// The function does _not_ delete non-empty data structures. So deleting `array` from
    ///
    /// ```toml
    /// array = [ 1 ]
    /// ```
    ///
    /// does _not_ work.
    ///
    /// # Return value
    ///
    /// If the delete operation worked correctly, `Ok(Option<Value>)` is returned.
    ///
    /// The `Option<Value>` part is `None` if no value was actually removed as there was no value
    /// there. For example, if you're deleting `table.a` and the Table `table` has no key `a`, then
    /// `Ok(None)` is returned. Also, if you're deleting from an Array, but there is nothing in the
    /// array, or the array is shorter than the index you're deleting.
    /// If the delete operation actually removed something from the toml document, this value is
    /// returned as `Ok(Some(Value))`.
    ///
    /// On failure, `Err(e)` is returned
    ///
    fn delete_with_seperator(&mut self, query: &String, sep: char) -> Result<Option<Value>>;

    /// Extension function for inserting a value from the current toml::Value document
    ///
    /// See documentation of `TomlValueinsertExt::insert_with_seperator`
    fn delete(&mut self, query: &String) -> Result<Option<Value>> {
        self.delete_with_seperator(query, '.')
    }

}

impl TomlValueDeleteExt for Value {

    fn delete_with_seperator(&mut self, query: &String, sep: char) -> Result<Option<Value>> {
        use resolver::mut_resolver::resolve;
        use std::ops::Index;

        let mut tokens = try!(tokenize_with_seperator(query, sep));
        let last_token = tokens.pop_last();

        /// Check whether a structure (Table/Array) is empty. If the Value has not these types,
        /// the default value is returned
        #[inline]
        fn is_empty(val: Option<&Value>, default: bool) -> bool {
            val.map(|v| match v {
                    &Value::Table(ref tab) => tab.is_empty(),
                    &Value::Array(ref arr) => arr.is_empty(),
                    _                      => default
                })
                .unwrap_or(default)
        }

        #[inline]
        fn is_table(val: Option<&Value>) -> bool {
            val.map(|v| is_match!(v, &Value::Table(_))).unwrap_or(false)
        }

        #[inline]
        fn is_array(val: Option<&Value>) -> bool {
            val.map(|v| is_match!(v, &Value::Array(_))).unwrap_or(false)
        }

        #[inline]
        fn name_of_val(val: Option<&Value>) -> &'static str {
            val.map(|v| match v {
                &Value::Array(_)    => "Array",
                &Value::Boolean(_)  => "Boolean",
                &Value::Datetime(_) => "Datetime",
                &Value::Float(_)    => "Float",
                &Value::Integer(_)  => "Integer",
                &Value::String(_)   => "String",
                &Value::Table(_)    => "Table",
            }).unwrap_or("None")
        }

        if last_token.is_none() {
            match self {
                &mut Value::Table(ref mut tab) => {
                    match tokens {
                        Token::Identifier { ident, .. } => {
                            if is_empty(tab.get(&ident), true) {
                                Ok(tab.remove(&ident))
                            } else {
                                if is_table(tab.get(&ident)) {
                                    let kind = ErrorKind::CannotDeleteNonEmptyTable(ident.clone());
                                    Err(Error::from(kind))
                                } else if is_array(tab.get(&ident)) {
                                    let kind = ErrorKind::CannotDeleteNonEmptyArray(ident.clone());
                                    Err(Error::from(kind))
                                } else {
                                    let act = name_of_val(tab.get(&ident));
                                    let tbl = "table";
                                    let k   = ErrorKind::CannotAccessBecauseTypeMismatch(tbl, act);
                                    Err(Error::from(k))
                                }
                            }
                        },
                        _ => Ok(None)
                    }
                },
                _ => unimplemented!()
            }
        } else {
            let mut val = try!(resolve(self, &tokens));
            let last_token = last_token.unwrap();
            match val {
                &mut Value::Table(ref mut tab) => {
                    match *last_token {
                        Token::Identifier { ref ident, .. } => {
                            if is_empty(tab.get(ident), true) {
                                Ok(tab.remove(ident))
                            } else {
                                if is_table(tab.get(ident)) {
                                    let kind = ErrorKind::CannotDeleteNonEmptyTable(ident.clone());
                                    Err(Error::from(kind))
                                } else if is_array(tab.get(ident)) {
                                    let kind = ErrorKind::CannotDeleteNonEmptyArray(ident.clone());
                                    Err(Error::from(kind))
                                } else {
                                    let act = name_of_val(tab.get(ident));
                                    let tbl = "table";
                                    let k   = ErrorKind::CannotAccessBecauseTypeMismatch(tbl, act);
                                    Err(Error::from(k))
                                }
                            }
                        },
                        Token::Index { idx, .. } => {
                            unimplemented!()
                        },
                    }
                },
                &mut Value::Array(ref mut arr) => {
                    match *last_token {
                        Token::Identifier { ref ident, .. } => {
                            unimplemented!()
                        },
                        Token::Index { idx, .. } => {
                            if is_empty(Some(&arr.index(idx)), true) {
                                Ok(Some(arr.remove(idx)))
                            } else {
                                unimplemented!()
                            }
                        },
                    }
                },
                _ => {
                    unimplemented!()
                }
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use error::*;
    use toml::Value;
    use toml::from_str as toml_from_str;

    #[test]
    fn test_delete_from_empty_document() {
        let mut toml : Value = toml_from_str("").unwrap();

        let res = toml.delete_with_seperator(&String::from("a"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn test_delete_from_empty_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table.a"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn test_delete_integer() {
        let mut toml : Value = toml_from_str(r#"
        value = 1
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("value"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(is_match!(res, Value::Integer(1)));
    }

    #[test]
    fn test_delete_string() {
        let mut toml : Value = toml_from_str(r#"
        value = "foo"
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("value"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(is_match!(res, Value::String(_)));
        match res {
            Value::String(ref s) => assert_eq!("foo", s),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_delete_empty_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(is_match!(res, Value::Table(_)));
        match res {
            Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_delete_empty_array() {
        let mut toml : Value = toml_from_str(r#"
        array = []
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("array"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(is_match!(res, Value::Array(_)));
        match res {
            Value::Array(ref a) => assert!(a.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_delete_nonempty_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        a = 1
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table"), '.');

        assert!(res.is_err());

        let res = res.unwrap_err();
        assert!(is_match!(res.kind(), &ErrorKind::CannotDeleteNonEmptyTable(_)));
    }

    #[test]
    fn test_delete_nonempty_array() {
        let mut toml : Value = toml_from_str(r#"
        array = [ 1 ]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("array"), '.');

        assert!(res.is_err());

        let res = res.unwrap_err();
        assert!(is_match!(res.kind(), &ErrorKind::CannotDeleteNonEmptyArray(_)));
    }

    #[test]
    fn test_delete_int_from_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        int = 1
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table.int"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Integer(1))));
    }

    #[test]
    fn test_delete_array_from_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        array = []
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table.array"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Array(_))));
    }

    #[test]
    fn test_delete_int_from_array_from_table() {
        let mut toml : Value = toml_from_str(r#"
        [table]
        array = [ 1 ]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("table.array.[0]"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Integer(1))));
    }

    #[test]
    fn test_delete_int_from_array() {
        let mut toml : Value = toml_from_str(r#"
        array = [ 1 ]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("array.[0]"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Integer(1))));
    }

    #[test]
    fn test_delete_int_from_table_from_array() {
        let mut toml : Value = toml_from_str(r#"
        array = [ { table = { int = 1 } } ]
        "#).unwrap();

        let res = toml.delete_with_seperator(&String::from("array.[0].table.int"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Integer(1))));
    }

    #[test]
    fn test_delete_from_array_value() {
        use read::TomlValueReadExt;

        let mut toml : Value = toml_from_str(r#"
        array = [ 1 ]
        "#).unwrap();

        let mut ary = toml.read_mut(&String::from("array")).unwrap();
        let res     = ary.delete_with_seperator(&String::from("[0]"), '.');

        assert!(res.is_ok());

        let res = res.unwrap();
        assert!(is_match!(res, Some(Value::Integer(1))));
    }

    #[test]
    fn test_delete_from_int_value() {
        use read::TomlValueReadExt;

        let mut toml : Value = toml_from_str(r#"
        array = [ 1 ]
        "#).unwrap();

        let mut ary = toml.read_mut(&String::from("array.[0]")).unwrap();
        let res     = ary.delete_with_seperator(&String::from("nonexist"), '.');

        assert!(res.is_err());

        let res = res.unwrap_err();
        assert!(is_match!(res.kind(), &ErrorKind::QueryingValueAsTable(_)));
    }

    #[test]
    fn test_delete_index_from_non_array() {
        use read::TomlValueReadExt;

        let mut toml : Value = toml_from_str(r#"
        array = 1
        "#).unwrap();

        let mut ary = toml.read_mut(&String::from("array")).unwrap();
        let res     = ary.delete_with_seperator(&String::from("[0]"), '.');

        assert!(res.is_err());

        let res = res.unwrap_err();
        assert!(is_match!(res.kind(), &ErrorKind::CannotAccessBecauseTypeMismatch(_, _)));
    }

}

