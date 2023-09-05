//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

use crate::error::{Error, Result};
use crate::tokenizer::Token;
/// The query resolver that operates on the AST and the TOML object
use toml::{map::Map, Value};

pub fn resolve<'doc>(toml: &'doc mut Value, tokens: &Token) -> Result<&'doc mut Value> {
    // Cases:
    //
    //  1. Identifier, toml: table, ident present       -> traverse
    //  2. Identifier, toml: table, no indent present   -> create Table
    //      2.1 If next token                           -> traverse
    //      2.2 no next token                           -> return created Table
    //  3. Identifier, toml: array                      -> error
    //  4. Index, toml: table                           -> error
    //  5. Index, toml: array, idx present              -> traverse
    //  6. Index, toml: array, idx not present
    //      6.1 -> next token is ident                  -> push Table
    //      6.2 -> next token is index                  -> push Array
    //      then traverse

    match *tokens {
        Token::Identifier { ref ident, .. } => match toml {
            Value::Table(ref mut t) => {
                if t.contains_key(ident) {
                    match tokens.next() {
                        Some(next) => resolve(t.get_mut(ident).unwrap(), next),
                        None => t.get_mut(ident).ok_or_else(|| unreachable!()),
                    }
                } else {
                    match tokens.next() {
                        Some(next) => {
                            let subdoc = t.entry(ident.clone()).or_insert(Value::Table(Map::new()));
                            resolve(subdoc, next)
                        }
                        None => Ok(t.entry(ident.clone()).or_insert(Value::Table(Map::new()))),
                    }
                }
            }
            Value::Array(_) => Err(Error::NoIdentifierInArray(ident.clone())),
            _ => unimplemented!(),
        },
        Token::Index { idx, .. } => {
            match toml {
                Value::Table(_) => Err(Error::NoIndexInTable(idx)),
                Value::Array(ref mut ary) => {
                    if ary.len() > idx {
                        match tokens.next() {
                            Some(next) => resolve(ary.get_mut(idx).unwrap(), next),
                            None => ary.get_mut(idx).ok_or_else(|| unreachable!()),
                        }
                    } else if let Some(next) = tokens.next() {
                        match next {
                            Token::Identifier { .. } => {
                                ary.push(Value::Table(Map::new()));
                            }
                            Token::Index { .. } => {
                                ary.push(Value::Array(vec![]));
                            }
                        }
                        //resolve(toml, next)
                        panic!("Cannot do this")
                    } else {
                        unimplemented!()
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::resolve;
    use crate::tokenizer::*;
    use toml::from_str as toml_from_str;
    use toml::Value;

    macro_rules! do_resolve {
        ( $toml:ident => $query:expr ) => {
            resolve(
                &mut $toml,
                &tokenize_with_seperator(&String::from($query), '.').unwrap(),
            )
        };
    }

    #[test]
    fn test_resolve_empty_toml_simple_query() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(is_match!(result, Value::Table(_)));
        match result {
            Value::Table(ref tab) => assert!(tab.is_empty()),
            _ => unreachable!("Expected Table, got something else"),
        }
    }

    #[test]
    fn test_resolve_present_bool() {
        let mut toml = toml_from_str("example = true").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Boolean(true)));
    }

    #[test]
    fn test_resolve_present_integer() {
        let mut toml = toml_from_str("example = 1").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(1)));
    }

    #[test]
    fn test_resolve_present_float() {
        let mut toml = toml_from_str("example = 1.0").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Float(_)));
        assert_eq!(result.as_float(), Some(1.0));
    }

    #[test]
    fn test_resolve_present_string() {
        let mut toml = toml_from_str("example = 'string'").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::String(_)));
        match result {
            Value::String(ref s) => assert_eq!("string", s),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_bools() {
        let mut toml = toml_from_str("example = [ true, false ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Array(_)));
        match result {
            Value::Array(ref ary) => {
                assert_eq!(ary[0], Value::Boolean(true));
                assert_eq!(ary[1], Value::Boolean(false));
            }
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_integers() {
        let mut toml = toml_from_str("example = [ 1, 1337 ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Array(_)));
        match result {
            Value::Array(ref ary) => {
                assert_eq!(ary[0], Value::Integer(1));
                assert_eq!(ary[1], Value::Integer(1337));
            }
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_floats() {
        let mut toml = toml_from_str("example = [ 1.0, 133.25 ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Array(_)));
        match result {
            Value::Array(ref ary) => {
                assert!(is_match!(ary[0], Value::Float(_)));
                assert_eq!(ary[0].as_float(), Some(1.0));
                assert!(is_match!(ary[1], Value::Float(_)));
                assert_eq!(ary[1].as_float(), Some(133.25));
            }
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_array_index_query_1() {
        let mut toml = toml_from_str("example = [ 1 ]").unwrap();
        let result = do_resolve!(toml => "example.[0]");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(1)));
    }

    #[test]
    fn test_resolve_array_index_query_2() {
        let mut toml = toml_from_str("example = [ 1, 2, 3, 4, 5 ]").unwrap();
        let result = do_resolve!(toml => "example.[4]");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(5)));
    }

    #[test]
    fn test_resolve_table_element_query() {
        let mut toml = toml_from_str(
            r#"
        [table]
        value = 42
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "table.value");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(42)));
    }

    #[test]
    fn test_resolve_table_with_many_elements_element_query() {
        let mut toml = toml_from_str(
            r#"
        [table]
        value1 = 42
        value2 = 43
        value3 = 44
        value4 = 45
        value5 = 46
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "table.value1");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(42)));
    }

    #[test]
    fn test_resolve_table_array_query() {
        let mut toml = toml_from_str(
            r#"
        [table]
        value1 = [ 42.0, 50.0 ]
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "table.value1");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Array(_)));
        match result {
            Value::Array(ref ary) => {
                assert!(is_match!(ary[0], Value::Float(_)));
                assert_eq!(ary[0].as_float(), Some(42.0));
                assert!(is_match!(ary[1], Value::Float(_)));
                assert_eq!(ary[1].as_float(), Some(50.0));
            }
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_table_array_element_query() {
        let mut toml = toml_from_str(
            r#"
        [table]
        value1 = [ 42 ]
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "table.value1.[0]");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Integer(42)));
    }

    #[test]
    fn test_resolve_multi_table_query() {
        let mut toml = toml_from_str(
            r#"
        [table0]
        value = [ 1 ]
        [table1]
        value = [ "Foo" ]
        [table2]
        value = [ 42.0 ]
        [table3]
        value = [ true ]
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "table1.value.[0]");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::String(_)));
        match result {
            Value::String(ref s) => assert_eq!("Foo", s),
            _ => panic!("What just happened?"),
        }
    }

    static FRUIT_TABLE: &str = r#"
    [[fruit.blah]]
      name = "apple"

      [fruit.blah.physical]
        color = "red"
        shape = "round"

    [[fruit.blah]]
      name = "banana"

      [fruit.blah.physical]
        color = "yellow"
        shape = "bent"
    "#;

    #[test]
    fn test_resolve_array_table_query_1() {
        let mut toml = toml_from_str(FRUIT_TABLE).unwrap();
        let result = do_resolve!(toml => "fruit.blah.[0].name");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::String(_)));
        match result {
            Value::String(ref s) => assert_eq!("apple", s),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_array_table_query_2() {
        let mut toml = toml_from_str(FRUIT_TABLE).unwrap();
        let result = do_resolve!(toml => "fruit.blah.[0].physical");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Table(_)));
        match result {
            Value::Table(ref tab) => {
                match tab.get("color") {
                    Some(Value::String(ref s)) => assert_eq!("red", s),
                    _ => unreachable!(),
                }
                match tab.get("shape") {
                    Some(Value::String(ref s)) => assert_eq!("round", s),
                    _ => unreachable!(),
                }
            }
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_on_result() {
        let mut toml = toml_from_str(FRUIT_TABLE).unwrap();
        let result = do_resolve!(toml => "fruit.blah.[1].physical");

        assert!(result.is_ok());
        let result = result.unwrap();

        let tokens = tokenize_with_seperator(&String::from("color"), '.').unwrap();
        let result = resolve(result, &tokens);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::String(_)));
        match result {
            Value::String(ref s) => assert_eq!("yellow", s),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_empty_table() {
        let mut toml = toml_from_str(
            r#"
        [example]
        "#,
        )
        .unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, Value::Table(_)));
        match result {
            Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_member_of_empty_table() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example.foo");

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(is_match!(result, Value::Table(_)));
        match result {
            Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_index_in_table() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example.[0]");

        // TODO: Array creating is not yet implemented properly
        assert!(result.is_err());

        //assert!(result.is_ok());
        //let result = result.unwrap();

        //assert!(is_match!(result, Value::Array(_)));
        //match result {
        //    Value::Array(ref a) => assert!(a.is_empty()),
        //    _                        => panic!("What just happened?"),
        //}
    }

    #[test]
    fn test_resolve_query_identifier_in_array() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example.foo.bar");

        assert!(result.is_ok());
        let result = result.unwrap();

        match result {
            Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_value_as_table() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example.foo.bar");

        assert!(result.is_ok());
        let result = result.unwrap();

        match result {
            Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_query_value_as_array() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example.foo.[0]");

        // TODO: Array creating is not yet implemented properly
        assert!(result.is_err());

        //assert!(result.is_ok());
        //let result = result.unwrap();

        //match result {
        //    Value::Array(ref a) => assert!(a.is_empty()),
        //    _                        => panic!("What just happened?"),
        //}
    }
}
