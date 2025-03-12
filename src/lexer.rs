use crate::ast::SKI;
use crate::ast::{self, SKIErr};
use std::vec::Vec;
#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    SToken,
    KToken,
    IToken,
    Parens(Vec<Token>),
}

pub fn tokenize_ski(inp: &str) -> Result<Vec<Token>, ast::SKIErr> {
    let mut toks = Vec::new();
    let mut characters = inp.chars();

    while let Some(c) = characters.next() {
        match c{
        'S' => {
            toks.push(Token::SToken);
        }
        'K' => {
            toks.push(Token::KToken);
        }
        'I' => {
            toks.push(Token::IToken);
        }
        '(' => {
            let mut open = 1;
            let mut substr = String::from("");

            while open > 0 {
                match characters.next() {
                    None => {
                        return Err(ast::SKIErr::SyntaxError(String::from(
                            "unclosed parentheses",
                        )));
                    }
                    Some('(') => {
                        open += 1;
                        substr.push('(');
                    }
                    Some(')') => {
                        open -= 1;
                        if open > 0 {
                            substr.push(')');
                        }
                    }
                    Some(c) => {
                        substr.push(c);
                    }
                }
            }
            toks.push(Token::Parens(tokenize_ski(&substr)?));
        }
        ')' => {
            return Err(ast::SKIErr::SyntaxError(String::from(
                "unmatched closing parentheses",
            )))
        }
        _ => {
            return Err(ast::SKIErr::ParseError(String::from(
                "not a SKI primitive",
            )))
        }

       
    }}
    return Ok(toks);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_succeeds_with_k_primitive() {
        assert_eq!(tokenize_ski(&String::from("K")), Ok(vec![Token::KToken]));
    }
    
    #[test]
    fn tokenize_succeeds_with_ski() {
        assert_eq!(tokenize_ski(&String::from("SKI")), Ok(vec![Token::SToken,Token::KToken,Token::IToken]));
    }
    #[test]
    fn tokenize_succeeds_with_kis() {
        assert_eq!(tokenize_ski(&String::from("KIS")), Ok(vec![Token::KToken,Token::IToken,Token::SToken]));
    }
    #[test]
    fn tokenize_succeeds_with_parens() {
        assert_eq!(tokenize_ski(&String::from("S(KI)")), Ok(vec![Token::SToken,Token::Parens(vec![Token::KToken,Token::IToken])]));
    }
    
    #[test]
    fn tokenize_succeeds_with_nested_parens() {
        assert_eq!(tokenize_ski(&String::from("S(K(I))")), Ok(vec![Token::SToken,Token::Parens(vec![Token::KToken,Token::Parens(vec![Token::IToken])])]));
    }
    #[test]
    fn tokenize_succeeds_with_multiple_parens() {
        assert_eq!(tokenize_ski(&String::from("S((K)(I))")), Ok(vec![Token::SToken,Token::Parens(vec![Token::Parens(vec![Token::KToken]),Token::Parens(vec![Token::IToken])])]));
    }
    #[test]
    fn tokenize_fails_with_unclosed_parens() {
        assert_eq!(tokenize_ski(&String::from("S(KI")), Err(ast::SKIErr::SyntaxError(String::from("unclosed parentheses"))));
    }
    #[test]
    fn tokenize_fails_with_incorrect_inside_parens() {
        assert_eq!(tokenize_ski(&String::from("S(KT)K")), Err(ast::SKIErr::ParseError(String::from("not a SKI primitive"))));
    }
    #[test]
    fn tokenize_fails_with_incorrect_nested_parens() {
        assert_eq!(tokenize_ski(&String::from("S(K(I)K")), Err(ast::SKIErr::SyntaxError(String::from("unclosed parentheses"))));
    }
    #[test]
    fn tokenize_fails_with_non_ski_primitive() {
        assert_eq!(tokenize_ski(&String::from("STKI")), Err(ast::SKIErr::ParseError(String::from("not a SKI primitive"))));
    }
}