use crate::ast;
use crate::ast::SKI;
use crate::eval;

/// parses a single char as a SKI primitive or else returns a SKIErr
pub fn parse_single_char(inp: &char) -> Result<ast::SKI, ast::SKIErr> {
    match inp {
        'I' => Ok(ast::SKI::I),
        'K' => Ok(ast::SKI::K),
        'S' => Ok(ast::SKI::S),
        _ => Err(ast::SKIErr::ParseError(String::from("no SKI primitive"))),
    }
}
/// parses an optional value of type char and returns error on None
pub fn maybe_parse_single_char(inp: &Option<char>) -> Result<ast::SKI, ast::SKIErr> {
    match inp {
        None => Err(ast::SKIErr::ParseError(String::from("no input"))),
        Some(c) => parse_single_char(c),
    }
}
/// parses the App variant of the ski combinator
pub fn parse_app(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    let mut open_parens: Vec<usize> = inp
        .char_indices()
        .filter(|(_i, c)| *c == '(')
        .map(|(i, _c)| i)
        .collect();
    let mut close_parens: Vec<usize> = inp
        .char_indices()
        .filter(|(_i, c)| *c == ')')
        .map(|(i, _c)| i)
        .collect();

    if open_parens.len() > close_parens.len() {
        return Err(ast::SKIErr::SyntaxError(String::from(
            "unclosed parentheses",
        )));
    }
    if open_parens.len() < close_parens.len() {
        return Err(ast::SKIErr::SyntaxError(String::from(
            "unmatched closing parentheses",
        )));
    }
    if open_parens.is_empty() {
        match maybe_parse_single_char(&inp.chars().last()) {
            Err(e) => Err(e),
            Ok(skiprim) => match parse_ski(&inp[..inp.len() - 1]) {
                Err(e) => Err(e),
                Ok(skiexpr) => Ok(ast::SKI::app(skiexpr, skiprim)),
            },
        }
    } else {
        let (matched_parens_open, matched_parens_close) = match_parens(&open_parens, &close_parens);
        //if the parens are all the way around an expression, they can be removed
        if matched_parens_open == 0 {
            if matched_parens_close == inp.len() - 1 {
                return parse_ski(&inp[1..inp.len() - 1]);
            } else {
                //consider if it is actually valid to index into string slices like this
                return parse_ski(
                    &(inp[1..matched_parens_close].to_owned()
                        + &inp[matched_parens_close + 1..inp.len()]),
                );
            }
        }
        if matched_parens_close == inp.len() - 1 {
            return Ok(ast::SKI::app(
                parse_ski(&inp[..matched_parens_open])?,
                parse_ski(&inp[matched_parens_open + 1..matched_parens_close])?,
            ));
        } else {
            match match_all_parens(&mut open_parens, &mut close_parens){
                None => Err(ast::SKIErr::SyntaxError(String::from(
                    "unmatched parentheses",
                ))),
                Some(matched_parens)=>{
            
                let mut blocks = identify_blocks(&matched_parens, &inp);
                blocks.reverse();

            return create_app(&blocks, &inp);
        }
        }
    }
}}
pub fn create_app(blocks: &Vec<(usize, usize)>, skiexp: &str) -> Result<ast::SKI, ast::SKIErr> {
    if blocks.len() == 1 {
        parse_ski(&skiexp[blocks[0].0..blocks[0].1])
    } else {
        match create_app(&blocks[1..].to_vec(), &skiexp) {
            Err(e) => Err(e),
            Ok(ski) => Ok(ast::SKI::app(
                ski,
                parse_ski(&skiexp[blocks[0].0..blocks[0].1])?,
            )),
        }
    }
}
/// this function creates blocks of combinators
pub fn identify_blocks(
    matched_parens: &Vec<(usize,usize)>,
    
    skiexp: &str,
) -> Vec<(usize, usize)> {
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
///this function tries to match up parentheses
pub fn match_parens(open_parens: &Vec<usize>, close_parens: &Vec<usize>) -> (usize, usize) {
    let mut open_iter = open_parens.iter();
    let mut close_iter = close_parens.iter();
    let open = open_iter.next();
    match open {
        None => (0, 0),
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
///this function tries to match up all parentheses
pub fn match_all_parens(open_parens: & mut Vec<usize>, close_parens: & mut Vec<usize>) -> Option<Vec<(usize, usize)> >{
    let mut matched_parens = Vec::new();
    if open_parens.len() != close_parens.len(){
        return None;
    }
    while ! open_parens.is_empty(){
        let (matched_open, matched_close) = match_parens(&open_parens, &close_parens);
        matched_parens.push((matched_open,matched_close));
        open_parens.retain(|&x| x != matched_open);
        close_parens.retain(|&x| x != matched_close);
    }
    Some(matched_parens)

}
/// parse any SKI variant
pub fn parse_ski(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    match inp.chars().count() {
        0 => Err(ast::SKIErr::ParseError(String::from("Empty input"))),
        1 => parse_single_char(&inp.chars().next().unwrap()), // this unwrap should be fine as we already checked the length.
        _ => parse_app(inp),
    }
}
/// parsing, then evaluating
pub fn parse_and_eval(inp: &str) -> Result<ast::SKI, ast::SKIErr> {
    parse_ski(inp).map(eval::eval)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_succeeds_with_k_primitive() {
        assert_eq!(parse_ski(&String::from("K")), Ok(ast::SKI::K));
    }
    #[test]
    fn parse_succeeds_with_i_primitive() {
        assert_eq!(parse_ski(&String::from("I")), Ok(ast::SKI::I));
    }
    #[test]
    fn parse_app_succeeds_with_kii() {
        assert_eq!(
            parse_app(&String::from("KII")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::I
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_kis() {
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
    fn parse_ski_succeeds_with_parens_followed_by_multiple_combinators() {
        assert_eq!(
            parse_ski(&String::from("(KI)KS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::app(ast::SKI::K, ast::SKI::I), ast::SKI::K),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parens_at_start_has_no_effect() {
        assert_eq!(
            parse_ski(&String::from("KIKS")),
            parse_ski(&String::from("(KI)KS"))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_no_parens_followed_by_multiple_combinators() {
        assert_eq!(
            parse_ski(&String::from("KIKS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::app(ast::SKI::K, ast::SKI::I), ast::SKI::K),
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
    fn parse_ski_succeeds_with_parens_middle_longer_expr() {
        assert_eq!(
            parse_ski(&String::from("K(II)SK")),
            Ok(ast::SKI::app(
                ast::SKI::app(
                    ast::SKI::app(ast::SKI::K, ast::SKI::app(ast::SKI::I, ast::SKI::I)),
                    ast::SKI::S
                ),
                ast::SKI::K,
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
    fn parse_and_eval_succeeds_with_primitives_in_parens() {
        assert_eq!(parse_and_eval(&String::from("K(I)(S)")), Ok(ast::SKI::I));
    }
    #[test]
    fn parse_and_eval_fails_with_non_primitive() {
        assert_eq!(
            parse_and_eval(&String::from("KIT")),
            Err(ast::SKIErr::ParseError(String::from("no SKI primitive")))
        );
        assert_eq!(
            parse_and_eval(&String::from("T")),
            Err(ast::SKIErr::ParseError(String::from("no SKI primitive")))
        );
        assert_eq!(
            parse_and_eval(&String::from("AKI")),
            Err(ast::SKIErr::ParseError(String::from("no SKI primitive")))
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
