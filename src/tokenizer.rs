/// The tokenizer for the query interpreter

use error::*;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier {
        ident: String,
        next: Option<Rc<Token>>
    },

    Index {
        idx: usize,
        next: Option<Rc<Token>>
    }
}

impl Token {

    pub fn set_next(&mut self, token: Token) {
        match self {
            &mut Token::Identifier { next: ref mut next, .. } => *next = Some(Rc::new(token)),
            &mut Token::Index { next: ref mut next, .. }      => *next = Some(Rc::new(token)),
        }
    }

    pub fn identifier(&self) -> Option<&String> {
        match self {
            &Token::Identifier { ident: ref ident, .. } => Some(&ident),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn idx(&self) -> usize {
        match self {
            &Token::Index { idx: i, .. } => i,
            _ => unreachable!(),
        }
    }
}

pub fn tokenize_with_seperator(query: &String, seperator: char) -> Result<Token> {
    use std::str::Split;

    /// Creates a Token object from a string
    ///
    /// # Panics
    ///
    /// * If the internal regex does not compile (should never happen)
    /// * If the token is non-valid (that is, a array index with a non-i64)
    /// * If the regex does not find anything
    /// * If the integer in the brackets (`[]`) cannot be parsed to a valid i64
    ///
    /// # Incorrect behaviour
    ///
    /// * If the regex finds multiple captures
    ///
    /// # Returns
    ///
    /// The `Token` object with the correct identifier/index for this token and no next token.
    ///
    fn mk_token_object(s: &str) -> Result<Token> {
        use regex::Regex;
        use std::str::FromStr;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\[\d+\]$").unwrap();
        }

        if !has_array_brackets(s) {
            return Ok(Token::Identifier { ident: String::from(s), next: None });
        }

        match RE.captures(s) {
            None => return Err(Error::from(ErrorKind::ArrayAccessWithoutIndex)),
            Some(captures) => {
                match captures.get(0) {
                    None => Ok(Token::Identifier { ident: String::from(s), next: None }),
                    Some(mtch) => {
                        let mtch = without_array_brackets(mtch.as_str());
                        let i : usize = FromStr::from_str(&mtch).unwrap(); // save because regex
                        Ok(Token::Index {
                            idx: i,
                            next: None,
                        })
                    }
                }
            }
        }
    }

    /// Check whether a str begins with '[' and ends with ']'
    fn has_array_brackets(s: &str) -> bool {
        s.as_bytes()[0] == b'[' && s.as_bytes()[s.len() - 1] == b']'
    }

    /// Remove '[' and ']' from a str
    fn without_array_brackets(s: &str) -> String {
        s.replace("[","").replace("]","")
    }

    fn build_token_tree(split: &mut Split<char>, last: &mut Token) -> Result<()> {
        match split.next() {
            None        => { /* No more tokens */ }
            Some(token) => {
                if token.len() == 0 {
                    return Err(Error::from(ErrorKind::EmptyIdentifier));
                }
                let mut token = try!(mk_token_object(token));
                try!(build_token_tree(split, &mut token));
                last.set_next(token);
            }
        }
        Ok(())
    }

    if query.is_empty() {
        return Err(Error::from(ErrorKind::EmptyQueryError));
    }

    let mut tokens = query.split(seperator);

    match tokens.next() {
        None        => Err(Error::from(ErrorKind::EmptyQueryError)),
        Some(token) => {
            if token.len() == 0 {
                return Err(Error::from(ErrorKind::EmptyIdentifier));
            }
            let mut tok = try!(mk_token_object(token));
            let _       = try!(build_token_tree(&mut tokens, &mut tok));
            Ok(tok)
        }
    }
}

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
    fn test_tokenize_seperator_only() {
        let tokens = tokenize_with_seperator(&String::from("."), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::EmptyIdentifier { .. }));
    }

    #[test]
    fn test_tokenize_array_brackets_only() {
        let tokens = tokenize_with_seperator(&String::from("[]"), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::ArrayAccessWithoutIndex { .. }));
    }

    #[test]
    fn test_tokenize_identifiers_with_array_brackets_only() {
        let tokens = tokenize_with_seperator(&String::from("a.b.c.[]"), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::ArrayAccessWithoutIndex { .. }));
    }

    #[test]
    fn test_tokenize_identifiers_in_array_brackets() {
        let tokens = tokenize_with_seperator(&String::from("[a]"), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::ArrayAccessWithoutIndex { .. }));
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
                assert_eq!("b", next.deref().identifier().unwrap());
                match next.deref() {
                    &Token::Identifier { next: None, .. } => true,
                    _ => false
                }
            },
            _ => false,
        });
        assert_eq!("a", tokens.identifier().unwrap());
    }

    #[test]
    fn test_tokenize_ident_then_array_query() {
        let tokens = tokenize_with_seperator(&String::from("a.[0]"), '.');
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();

        assert_eq!("a", tokens.identifier().unwrap());
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

        assert_eq!("a", tokens.identifier().unwrap());

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

    #[test]
    fn test_tokenize_empty_token_after_good_token() {
        let tokens = tokenize_with_seperator(&String::from("a..b"), '.');
        assert!(tokens.is_err());
        let tokens = tokens.unwrap_err();

        let errkind = tokens.kind();
        assert!(is_match!(errkind, &ErrorKind::EmptyIdentifier { .. }));
    }

    quickcheck! {
        fn test_array_index(i: usize) -> bool {
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
