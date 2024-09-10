use crate::ast;
use crate::eval;

/// parses a single char as a SKI primitive or else returns a SKIErr
pub fn parse_single_char(inp: &char) -> Result<ast::SKI, ast::SKIErr> {
    match inp {
        'I' => Ok(ast::SKI::I),
        'K' => Ok(ast::SKI::K),
        'S' => Ok(ast::SKI::S),
        _ => Err(String::from("no SKI primitive")),
    }
}
/// parses an optional value of type char and returns error on None
pub fn maybe_parse_single_char(inp: &Option<char>) -> Result<ast::SKI, ast::SKIErr> {
    match inp {
        None => Err(String::from("no input")),
        Some(c) => parse_single_char(c),
    }
}
/// parses the App variant of the ski combinator
pub fn parse_app2(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    if inp.ends_with('(') {
        return Err(String::from("unclosed parentheses"));
    }
    if inp.ends_with(')') {
        if inp.starts_with('(') {
            return parse_ski(&inp[1..inp.len() - 1]);
        }
        for (i, c) in inp.char_indices() {
            if c == '(' {
                return Ok(ast::SKI::app(
                    parse_ski(&inp[..i])?,
                    parse_ski(&inp[i + 1..inp.len() - 1])?,
                ));
            }
        }
        return Err(String::from("unmatched closing parentheses"));
    } else {
        match maybe_parse_single_char(&inp.chars().last()) {
            Err(e) => return Err(e),
            Ok(skiprim) => match parse_ski(&inp[..inp.len() - 1]) {
                Err(e) => return Err(e),
                Ok(skiexpr) => return Ok(ast::SKI::app(skiexpr, skiprim)),
            },
        }
    }
}
pub fn parse_app(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    let open_parens: Vec<usize> = inp
        .char_indices()
        .filter(|(i, c)| *c == '(')
        .map(|(i, c)| i)
        .collect();
    let close_parens: Vec<usize> = inp
        .char_indices()
        .filter(|(i, c)| *c == ')')
        .map(|(i, c)| i)
        .collect();

    if open_parens.len() != close_parens.len() {
        return Err(String::from("unmatched closing parentheses"));
    }
    if open_parens.len() == 0 {
        match maybe_parse_single_char(&inp.chars().last()) {
            Err(e) => return Err(e),
            Ok(skiprim) => match parse_ski(&inp[..inp.len() - 1]) {
                Err(e) => return Err(e),
                Ok(skiexpr) => return Ok(ast::SKI::app(skiexpr, skiprim)),
            },
        }
    } else {
        let (matched_parens_open, matched_parens_close) = match_parens(open_parens, close_parens);
        if matched_parens_open == 0 {
            if matched_parens_close == inp.len() - 1 {
                return parse_ski(&inp[1..inp.len() - 1]);
            } else {
                return Ok(ast::SKI::app(
                    parse_ski(&inp[..matched_parens_close])?,
                    parse_ski(&inp[matched_parens_close + 1..inp.len()])?,
                ));
            }
        }
        if matched_parens_close == inp.len() - 1 {
            return Ok(ast::SKI::app(
                parse_ski(&inp[..matched_parens_open])?,
                parse_ski(&inp[matched_parens_open + 1..matched_parens_close])?,
            ));
        } else {
            return Ok(ast::SKI::app(
                parse_ski(&inp[..matched_parens_open])?,
                ast::SKI::app(
                    parse_ski(&inp[matched_parens_open + 1..matched_parens_close])?,
                    parse_ski(&inp[matched_parens_close + 1..inp.len()])?,
                ),
            ));
        }
    }
}

pub fn match_parens(open_parens: Vec<usize>, close_parens: Vec<usize>) -> (usize, usize) {
    let mut open_iter = open_parens.iter();
    let mut close_iter = close_parens.iter();
    let open = open_iter.next();
    match open {
        None => return (0, 0),
        Some(&op) => loop {
            let next_open = open_iter.next();
            let next_close = close_iter.next();
            match next_open {
                None => return (op, *next_close.unwrap()),
                Some(o) => {
                    if o > next_close.unwrap() {
                        return (op, *next_close.unwrap());
                    } else {
                        continue;
                    }
                }
            }
        },
    }
}
/// parse any SKI variant
pub fn parse_ski(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    match inp.chars().count() {
        0 => Err(String::from("Empty input")),
        1 => parse_single_char(&inp.chars().next().unwrap()), // this unwrap should be fine as we already checked the length.
        _ => parse_app(inp),
    }
}
pub fn parse_and_eval(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    parse_ski(inp).map(|ski| eval::eval(ski))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_app_succeeds_with_kii() {
        assert_eq!(
            parse_app(&String::from("KIS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_kii() {
        assert_eq!(
            parse_ski(&String::from("KIS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_parens_first() {
        assert_eq!(
            parse_ski(&String::from("(KI)S")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_parens_last() {
        assert_eq!(
            parse_ski(&String::from("K(IS)")),
            Ok(ast::SKI::app(
                ast::SKI::K,
                ast::SKI::app(ast::SKI::I, ast::SKI::S)
            ))
        );
    }

    #[test]
    fn parse_ski_succeeds_with_parens_middle() {
        assert_eq!(parse_and_eval(&String::from("K(IS)K")), Ok(ast::SKI::S));
    }
    #[test]
    fn parse_and_eval_succeeds_with_kii() {
        assert_eq!(parse_and_eval(&String::from("KIS")), Ok(ast::SKI::I));
    }
    #[test]
    fn parse_and_eval_fails_with_non_primitive() {
        assert_eq!(
            parse_and_eval(&String::from("KIT")),
            Err(String::from("no SKI primitive"))
        );
        assert_eq!(
            parse_and_eval(&String::from("T")),
            Err(String::from("no SKI primitive"))
        );
        assert_eq!(
            parse_and_eval(&String::from("AKI")),
            Err(String::from("no SKI primitive"))
        );
    }
    #[test]
    fn parse_and_eval_fails_with_unclosed_parens() {
        assert_eq!(
            parse_and_eval(&String::from("K(I")),
            Err(String::from("unclosed parentheses"))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(IS(KI)")),
            Err(String::from("unclosed parentheses"))
        );
    }
    #[test]
    fn parse_and_eval_fails_with_unmatched_parens() {
        assert_eq!(
            parse_and_eval(&String::from("K(I))")),
            Err(String::from("unmatched closing parentheses"))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(I)K)")),
            Err(String::from("unmatched closing parentheses"))
        );
        assert_eq!(
            parse_and_eval(&String::from("K(IK)SK)")),
            Err(String::from("unmatched closing parentheses"))
        );
    }
}
