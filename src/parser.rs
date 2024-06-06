use crate::ast;
use crate::ast::eval;

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
pub fn parse_app(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
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
/// parse any SKI variant
pub fn parse_ski(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    match inp.chars().count() {
        0 => Err(String::from("Empty input")),
        1 => parse_single_char(&inp.chars().next().unwrap()), // this unwrap should be fine as we already checked the length.
        _ => parse_app(inp),
    }
}
pub fn parse_and_eval(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    parse_ski(inp).map(|ski| eval(ski))
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
    fn parse_ski_succeeds_with_parens() {
        assert_eq!(
            parse_ski(&String::from("K(IS)")),
            Ok(ast::SKI::app(
                ast::SKI::K,
                ast::SKI::app(ast::SKI::I, ast::SKI::S)
            ))
        );
    }
    #[test]
    fn parse_and_eval_succeeds_with_kii() {
        assert_eq!(parse_and_eval(&String::from("KIS")), Ok(ast::SKI::I));
    }
}
