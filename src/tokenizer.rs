/// The tokenizer for the query interpreter

#[cfg(test)]
mod test {
    use error::*;
    use super::*;

    use std::ops::Deref;

    #[test]
    fn test_tokenize_empty_query_to_error() {
        let tokens = tokenize_with_seperator(&String::from(""), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::EmptyQueryError { .. }));
    }

    #[test]
    fn test_tokenize_single_token_query() {
        let tokens = tokenize_with_seperator(&String::from("example"), '.');
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert!(match tokens {
            Token::Identifier {
                ident: ident,
                next: None
            } => { 
                assert_eq!(String::from("example"), ident);
                true
            },
            _ => false,
        });
    }

    #[test]
    fn test_tokenize_double_token_query() {
        let tokens = tokenize_with_seperator(&String::from("a.b"), '.');
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert!(match tokens {
            Token::Identifier { next: Some(ref next), .. } => { 
                assert_eq!(1, Rc::strong_count(&next));
                assert_eq!("b", next.deref().identifier());
                match next.deref() {
                    &Token::Identifier { next: None, .. } => true,
                    _ => false
                }
            },
            _ => false,
        });
        assert_eq!("a", tokens.identifier());
    }

    #[test]
    fn test_tokenize_ident_then_array_query() {
        let tokens = tokenize_with_seperator(&String::from("a.[0]"), '.');
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!("a", tokens.identifier());
        assert!(match tokens {
            Token::Identifier { next: Some(ref next), .. } => {
                assert_eq!(1, Rc::strong_count(&next));
                match next.deref() {
                    &Token::Index { idx: 0, next: None } => true,
                    _ => false
                }
            },
            _ => false,
        });
    }

    #[test]
    fn test_tokenize_many_idents_then_array_query() {
        use std::ops::Deref;

        let tokens = tokenize_with_seperator(&String::from("a.b.c.[1000]"), '.');
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!("a", tokens.identifier());

        let expected =
            Token::Identifier {
                ident: String::from("a"),
                next: Some(Rc::new(Token::Identifier {
                    ident: String::from("b"),
                    next: Some(Rc::new(Token::Identifier {
                        ident: String::from("c"),
                        next: Some(Rc::new(Token::Index {
                            idx: 1000,
                            next: None,
                        })),
                    })),
                })),
            };

        assert_eq!(expected, tokens);
    }

    quickcheck! {
        fn test_array_index(i: i64) -> bool {
            match tokenize_with_seperator(&format!("[{}]", i), '.') {
                Ok(Token::Index {
                    idx: i,
                    next: None,
                }) => true,
                _ => false,
            }
        }
    }

}
