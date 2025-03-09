use crate::ast;
use crate::ast::SKI;
use crate::eval;
use crate::lexer;
use crate::lexer::Token;

///parses from Vec of tokens to enum SKI
pub fn parse_tokens(toks: &mut Vec<Token>) -> Result<ast::SKI, ast::SKIErr> {
    if toks.is_empty() {
        Err(ast::SKIErr::ParseError(String::from("empty input")))
    } else if toks.len() == 1 {
        parse_ski_token(&toks[0])
    } else {
        Ok(ast::SKI::app(
            parse_tokens(&mut toks[0..toks.len() - 1].to_vec())?,
            parse_ski_token(&toks[toks.len() - 1])?,
        ))
    }
}
///parses a single Token to SKI - used internally by parse_tokens
fn parse_ski_token(tok: &Token) -> Result<ast::SKI, ast::SKIErr> {
    match tok {
        Token::SToken => Ok(SKI::S),
        Token::IToken => Ok(SKI::I),
        Token::KToken => Ok(SKI::K),
        Token::Parens(x) => parse_tokens(&mut x.clone()),
    }
}
/// this function creates blocks of combinators
pub fn identify_blocks(matched_parens: &Vec<(usize, usize)>, skiexp: &str) -> Vec<(usize, usize)> {
    let mut block_starts = Vec::new();
    let mut block_ends = Vec::new();
    let mut curr_index = 0;
    for i in 0..matched_parens.len() {
        for j in curr_index..matched_parens[i].0 {
            block_starts.push(j);
            block_ends.push(j + 1);
        }
        block_starts.push(matched_parens[i].0 + 1);
        block_ends.push(matched_parens[i].1);
        curr_index = matched_parens[i].1 + 1;
    }
    for k in curr_index..skiexp.len() {
        block_starts.push(k);
        block_ends.push(k + 1);
    }
    block_starts
        .into_iter()
        .zip(block_ends.into_iter())
        .collect()
}

/// parsing, then evaluating
pub fn parse_and_eval(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    lexer::tokenize_ski(inp)
        .and_then(|mut toks| parse_tokens(&mut toks))
        .map(eval::eval)
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn parse_and_eval_succeeds_with_xor() {
        assert_eq!(
            parse_and_eval(&String::from("(K(K(KI)K)(KI))K((K(KI)K)K(KI))")),
            Ok(ast::SKI::Application(Box::new(ast::App {
                combinator: ast::SKI::K,
                arg: ast::SKI::I
            })))
        );
    }
    #[test]
    fn parse_and_eval_succeeds_with_parens_middle() {
        assert_eq!(parse_and_eval(&String::from("K(IS)K")), Ok(ast::SKI::S));
    }
    #[test]
    fn parse_and_eval_succeeds_with_k() {
        assert_eq!(parse_and_eval(&String::from("K")), Ok(ast::SKI::K));
    }
    #[test]
    fn parse_tokens_succeeds_with_kis() {
        assert_eq!(
            parse_tokens(&mut vec![Token::KToken, Token::IToken, Token::SToken]),
            Ok(ast::SKI::Application(Box::new(ast::App {
                combinator: ast::SKI::Application(Box::new(ast::App {
                    combinator: ast::SKI::K,
                    arg: ast::SKI::I
                })),
                arg: ast::SKI::S
            })))
        );
    }
    #[test]
    fn parse_and_eval_succeeds_with_kis() {
        assert_eq!(parse_and_eval(&String::from("KIS")), Ok(ast::SKI::I));
    }
    #[test]
    fn parse_and_eval_succeeds_with_primitives_in_parens() {
        assert_eq!(parse_and_eval(&String::from("K(I)(S)")), Ok(ast::SKI::I));
    }
    #[test]
    fn parse_and_eval_fails_with_non_primitive() {
        assert_eq!(
            parse_and_eval(&String::from("KIT")),
            Err(ast::SKIErr::ParseError(String::from("not a SKI primitive")))
        );
        assert_eq!(
            parse_and_eval(&String::from("T")),
            Err(ast::SKIErr::ParseError(String::from("not a SKI primitive")))
        );
        assert_eq!(
            parse_and_eval(&String::from("AKI")),
            Err(ast::SKIErr::ParseError(String::from("not a SKI primitive")))
        );
    }
    #[test]
    fn parse_and_eval_fails_with_unclosed_parens() {
        assert_eq!(
            parse_and_eval(&String::from("K(I")),
            Err(ast::SKIErr::SyntaxError(String::from(
                "unclosed parentheses"
            )))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(IS(KI)")),
            Err(ast::SKIErr::SyntaxError(String::from(
                "unclosed parentheses"
            )))
        );
    }
    #[test]
    fn parse_and_eval_fails_with_unmatched_parens() {
        assert_eq!(
            parse_and_eval(&String::from("K(I))")),
            Err(ast::SKIErr::SyntaxError(String::from(
                "unmatched closing parentheses"
            )))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(I)K)")),
            Err(ast::SKIErr::SyntaxError(String::from(
                "unmatched closing parentheses"
            )))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(IK)SK)")),
            Err(ast::SKIErr::SyntaxError(String::from(
                "unmatched closing parentheses"
            )))
        );
    }
}
