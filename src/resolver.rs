/// The query resolver that operates on the AST and the TOML object

use toml::Value;
use tokenizer::Token;
use error::*;

fn resolve(toml: &mut Value, tokens: Token) -> Result<&mut Value> {
    let ident = tokens.identifier().unwrap().clone();

    match toml {
        &mut Value::Table(ref mut t) => {
            match t.get_mut(&ident) {
                None    => Err(Error::from(ErrorKind::IdentifierNotFoundInDocument(ident))),
                Some(s) => Ok(s),
            }
        },
        _ => {
            Err(Error::from(ErrorKind::IdentifierNotFoundInDocument(ident)))
        }
    }
}

#[cfg(test)]
mod test {
    use toml::from_str as toml_from_str;
    use toml::Value;
    use tokenizer::*;
    use error::*;
    use super::resolve;

    macro_rules! do_resolve {
        ( $toml:ident => $query:expr ) => {
            resolve(&mut $toml, tokenize_with_seperator(&String::from($query), '.').unwrap())
        }
    }

    #[test]
    fn test_resolve_empty_toml_simple_query() {
        let mut toml = toml_from_str("").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_err());
        let result = result.unwrap_err();

        let errkind = result.kind();
        assert!(is_match!(errkind, &ErrorKind::IdentifierNotFoundInDocument { .. }));
    }

    #[test]
    fn test_resolve_present_bool() {
        let mut toml = toml_from_str("example = true").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Boolean(true)));
    }

    #[test]
    fn test_resolve_present_integer() {
        let mut toml = toml_from_str("example = 1").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Integer(1)));
    }

    #[test]
    fn test_resolve_present_float() {
        let mut toml = toml_from_str("example = 1.0").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Float(1.0)));
    }

    #[test]
    fn test_resolve_present_string() {
        let mut toml = toml_from_str("example = 'string'").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::String(_)));
        match result {
            &mut Value::String(ref s) => assert_eq!("string", s),
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_bools() {
        let mut toml = toml_from_str("example = [ true, false ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Array(_)));
        match result {
            &mut Value::Array(ref ary) => {
                assert_eq!(ary[0], Value::Boolean(true));
                assert_eq!(ary[1], Value::Boolean(false));
            },
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_integers() {
        let mut toml = toml_from_str("example = [ 1, 1337 ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Array(_)));
        match result {
            &mut Value::Array(ref ary) => {
                assert_eq!(ary[0], Value::Integer(1));
                assert_eq!(ary[1], Value::Integer(1337));
            },
            _ => panic!("What just happened?"),
        }
    }

    #[test]
    fn test_resolve_present_array_floats() {
        let mut toml = toml_from_str("example = [ 1.0, 133.25 ]").unwrap();
        let result = do_resolve!(toml => "example");

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(is_match!(result, &mut Value::Array(_)));
        match result {
            &mut Value::Array(ref ary) => {
                assert_eq!(ary[0], Value::Float(1.0));
                assert_eq!(ary[1], Value::Float(133.25));
            },
            _ => panic!("What just happened?"),
        }
    }

}
