/// The Toml Read extensions

#[cfg(feature = "typed")]
use std::fmt::Debug;

#[cfg(feature = "typed")]
use serde::{Serialize, Deserialize};
use toml::Value;

use crate::tokenizer::tokenize_with_seperator;
use crate::error::{Error, Result};

pub trait TomlValueReadExt<'doc> {

    /// Extension function for reading a value from the current toml::Value document
    /// using a custom seperator
    fn read_with_seperator(&'doc self, query: &str, sep: char) -> Result<Option<&'doc Value>>;

    /// Extension function for reading a value from the current toml::Value document mutably
    /// using a custom seperator
    fn read_mut_with_seperator(&'doc mut self, query: &str, sep: char) -> Result<Option<&'doc mut Value>>;

    /// Extension function for reading a value from the current toml::Value document
    fn read(&'doc self, query: &str) -> Result<Option<&'doc Value>> {
        self.read_with_seperator(query, '.')
    }

    /// Extension function for reading a value from the current toml::Value document mutably
    fn read_mut(&'doc mut self, query: &str) -> Result<Option<&'doc mut Value>> {
        self.read_mut_with_seperator(query, '.')
    }

    #[cfg(feature = "typed")]
    fn read_deserialized<'de, D: Deserialize<'de>>(&'doc self, query: &str) -> Result<Option<D>> {
        let raw = self.read(query)?;

        match raw {
            Some(value) => {
                let deserialized = value.clone().try_into().map_err(Error::TomlDeserialize)?;
                Ok(Some(deserialized))
            }
            None => Ok(None)
        }
    }

    #[cfg(feature = "typed")]
    fn read_partial<'a, P: Partial<'a>>(&'doc self) -> Result<Option<P::Output>> {
        self.read_deserialized::<P::Output>(P::LOCATION)
    }
}

/// Describes a _part_ of a document
#[cfg(feature = "typed")]
pub trait Partial<'a> {
    // The location ("section") of the header where to find the struct
    const LOCATION: &'static str;

    // The type which represents the data
    type Output: Serialize + Deserialize<'a> + Debug;
}


impl<'doc> TomlValueReadExt<'doc> for Value {

    fn read_with_seperator(&'doc self, query: &str, sep: char) -> Result<Option<&'doc Value>> {
        use crate::resolver::non_mut_resolver::resolve;

        tokenize_with_seperator(query, sep).and_then(move |tokens| resolve(self, &tokens, false))
    }

    fn read_mut_with_seperator(&'doc mut self, query: &str, sep: char) -> Result<Option<&'doc mut Value>> {
        use crate::resolver::mut_resolver::resolve;

        tokenize_with_seperator(query, sep).and_then(move |tokens| resolve(self, &tokens, false))
    }

}

pub trait TomlValueReadTypeExt<'doc> : TomlValueReadExt<'doc> {
    fn read_string(&'doc self, query: &str) -> Result<Option<String>>;
    fn read_int(&'doc self, query: &str)    -> Result<Option<i64>>;
    fn read_float(&'doc self, query: &str)  -> Result<Option<f64>>;
    fn read_bool(&'doc self, query: &str)   -> Result<Option<bool>>;
}

macro_rules! make_type_getter {
    ($fnname:ident, $rettype:ty, $typename:expr, $matcher:pat => $implementation:expr) => {
        fn $fnname(&'doc self, query: &str) -> Result<Option<$rettype>> {
            self.read_with_seperator(query, '.').and_then(|o| match o {
                $matcher => Ok(Some($implementation)),
                Some(o)  => Err(Error::TypeError($typename, crate::util::name_of_val(&o)).into()),
                None     => Ok(None),
            })
        }
    };
}

impl<'doc, T> TomlValueReadTypeExt<'doc> for T
    where T: TomlValueReadExt<'doc>
{
    make_type_getter!(read_string, String, "String", Some(&Value::String(ref obj)) => obj.clone());
    make_type_getter!(read_int, i64, "Integer", Some(&Value::Integer(obj)) => obj);
    make_type_getter!(read_float, f64, "Float", Some(&Value::Float(obj)) => obj);
    make_type_getter!(read_bool, bool, "Boolean", Some(&Value::Boolean(obj)) => obj);
}

#[cfg(test)]
mod test {
    use super::*;
    use toml::from_str as toml_from_str;

    #[test]
    fn test_read_empty() {
        let toml : Value = toml_from_str("").unwrap();

        let val  = toml.read_with_seperator(&String::from("a"), '.');

        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_none());
    }

    #[test]
    fn test_read_table() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read_with_seperator(&String::from("table"), '.');

        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_some());
        let val = val.unwrap();

        assert!(is_match!(val, &Value::Table(_)));
        match val {
            &Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_read_table_value() {
        let toml : Value = toml_from_str(r#"
        [table]
        a = 1
        "#).unwrap();

        let val  = toml.read_with_seperator(&String::from("table.a"), '.');

        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_some());
        let val = val.unwrap();

        assert!(is_match!(val, &Value::Integer(1)));
    }

    #[test]
    fn test_read_empty_table_value() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read_with_seperator(&String::from("table.a"), '.');
        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_none());
    }

    #[test]
    fn test_read_table_index() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read_with_seperator(&String::from("table.[0]"), '.');
        assert!(val.is_err());
        let err = val.unwrap_err();

        assert!(is_match!(err, Error::NoIndexInTable(_)));
    }

    ///
    ///
    /// Querying without specifying the seperator
    ///
    ///

    #[test]
    fn test_read_empty_without_seperator() {
        let toml : Value = toml_from_str("").unwrap();

        let val  = toml.read(&String::from("a"));
        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_none());
    }

    #[test]
    fn test_read_table_without_seperator() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read(&String::from("table"));

        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_some());
        let val = val.unwrap();

        assert!(is_match!(val, &Value::Table(_)));
        match val {
            &Value::Table(ref t) => assert!(t.is_empty()),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_read_table_value_without_seperator() {
        let toml : Value = toml_from_str(r#"
        [table]
        a = 1
        "#).unwrap();

        let val  = toml.read(&String::from("table.a"));

        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_some());
        let val = val.unwrap();

        assert!(is_match!(val, &Value::Integer(1)));
    }

    #[test]
    fn test_read_empty_table_value_without_seperator() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read(&String::from("table.a"));
        assert!(val.is_ok());
        let val = val.unwrap();

        assert!(val.is_none());
    }

    #[test]
    fn test_read_table_index_without_seperator() {
        let toml : Value = toml_from_str(r#"
        [table]
        "#).unwrap();

        let val  = toml.read(&String::from("table.[0]"));
        assert!(val.is_err());
        let err = val.unwrap_err();

        assert!(is_match!(err, Error::NoIndexInTable(_)));
    }

}

#[cfg(test)]
mod high_level_fn_test {
    use super::*;
    use toml::from_str as toml_from_str;

    #[test]
    fn test_read_table_value() {
        let toml : Value = toml_from_str(r#"
        [table]
        a = 1
        "#).unwrap();

        let val = toml.read_int("table.a").unwrap();

        assert_eq!(val.unwrap(), 1);
    }

    #[cfg(feature = "typed")]
    #[test]
    fn test_name() {
        let toml : Value = toml_from_str(r#"
        [table]
        a = 1
        "#).unwrap();

        let val: u32 = toml.read_deserialized("table.a").unwrap().unwrap();

        assert_eq!(val, 1);
    }

    #[cfg(feature = "typed")]
    #[test]
    fn test_deser() {
        use toml::map::Map;
        use crate::insert::TomlValueInsertExt;
        use crate::read::TomlValueReadExt;

        #[derive(Serialize, Deserialize, Debug)]
        struct Test {
            a: u64,
            s: String,
        }

        let mut toml = Value::Table(Map::new());
        let test     = Test {
            a: 15,
            s: String::from("Helloworld"),
        };

        assert!(toml.insert_serialized("table.value", test).unwrap().is_none());
        let _ : Test = toml.read_deserialized("table.value").unwrap().unwrap();

        assert!(true);
    }

}

#[cfg(all(test, feature = "typed"))]
mod partial_tests {
    use super::*;

    use toml::map::Map;
    use toml::Value;

    #[derive(Debug, Deserialize, Serialize)]
    struct TestObj {
        pub value: String,
    }

    impl<'a> Partial<'a> for TestObj {
        const LOCATION: &'static str = "foo";
        type Output                  = Self;
    }

    #[test]
    fn test_compiles() {
        let tbl = {
            let mut tbl = Map::new();
            tbl.insert(String::from("foo"), {
                let mut tbl = Map::new();
                tbl.insert(String::from("value"), Value::String(String::from("foobar")));
                Value::Table(tbl)
            });
            Value::Table(tbl)
        };

        let obj : TestObj = tbl.read_partial::<TestObj>().unwrap().unwrap();
        assert_eq!(obj.value, "foobar");
    }

}
