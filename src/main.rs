fn main() {
    println!("{:?}", ast::eval(ast::SKI::S));
    let app = ast::App {
        combinator: ast::SKI::I,
        arg: ast::SKI::K,
    };
    println!("{:?}", ast::eval(ast::SKI::Application(Box::new(app))));
    println!(
        "{:?}",
        parser::parse_and_eval("KI(IS)").unwrap_or(ast::SKI::K)
    );
}

mod ast {
    #[derive(Debug, Clone, PartialEq)]
    pub enum SKI {
        S,
        K,
        I,
        Application(Box<App>),
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct App {
        pub combinator: SKI,
        pub arg: SKI,
    }
    impl SKI {
        fn is_application(&self) -> bool {
            match self {
                SKI::Application(_) => true,
                _ => false,
            }
        }
        /// simple helper function for creating the Application variant of the SKI enum
        pub fn app(combinator: SKI, arg: SKI) -> SKI {
            SKI::Application(Box::new(App { combinator, arg }))
        }
    }
    pub type SKIErr = String;
    /// function eval reduces a ski expression to a simpler one if reducable
    pub fn eval(skiexp: SKI) -> SKI {
        match skiexp {
            SKI::Application(ref app) => match &app.combinator {
                SKI::I => eval(app.arg.clone()),

                SKI::Application(app2) => match &app2.combinator {
                    SKI::K => eval(app2.arg.clone()),
                    SKI::Application(app3) => match &app3.combinator {
                        SKI::S => eval(SKI::app(
                            SKI::app(app3.arg.clone(), app.arg.clone()),
                            SKI::app(app2.arg.clone(), app3.arg.clone()),
                        )),

                        _ => eval(SKI::app(eval(app.combinator.clone()), app.arg.clone())),
                    },

                    SKI::S => SKI::app(
                        SKI::app(SKI::S, eval(app2.arg.clone())),
                        eval(app.arg.clone()),
                    ),
                    _ => eval(SKI::app(eval(app.combinator.clone()), app.arg.clone())),
                },

                SKI::K => SKI::app(SKI::K, eval(app.arg.clone())),

                SKI::S => SKI::app(SKI::S, eval(app.arg.clone())),
            },

            ski => ski,
        }
    }
}
pub mod parser {
    use crate::ast::{self, eval};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// tests that i is irreducable
    fn i_evaluates_to_i() {
        let result = ast::eval(ast::SKI::I);
        assert_eq!(result, ast::SKI::I);
    }
    #[test]
    /// tests that K is irreducable
    fn k_evaluates_to_k() {
        let result = ast::eval(ast::SKI::K);
        assert_eq!(result, ast::SKI::K);
    }
    #[test]
    /// tests that II reduces to I
    fn ii_evaluates_to_i() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        assert_eq!(ast::eval(ii), ast::SKI::I);
    }
    #[test]
    /// tests III reduces to I
    fn iii_evaluates_to_i() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        let iii = ast::SKI::app(ii, ast::SKI::I);
        assert_eq!(ast::eval(iii), ast::SKI::I);
    }
    #[test]
    fn iik_evaluates_to_k() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        let iik = ast::SKI::app(ii, ast::SKI::K);
        assert_eq!(ast::eval(iik), ast::SKI::K);
    }
    #[test]
    /// tests that K returns the first argument
    fn kik_evaluates_to_i() {
        let ki = ast::SKI::app(ast::SKI::K, ast::SKI::I);
        let kik = ast::SKI::app(ki, ast::SKI::K);
        assert_eq!(ast::eval(kik), ast::SKI::I);
    }
    #[test]
    /// tests that KI(KI) reduces to I
    fn kiki_evaluates_to_i() {
        let ki = ast::SKI::app(ast::SKI::K, ast::SKI::I);
        let kiki = ast::SKI::app(ki.clone(), ki.clone());
        assert_eq!(ast::eval(kiki), ast::SKI::I);
    }
    #[test]
    /// tests that KIKS reduces to S
    fn kiks_evaluates_to_s() {
        let kiks = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::K, ast::SKI::I), ast::SKI::K),
            ast::SKI::S,
        );
        assert_eq!(ast::eval(kiks), ast::SKI::S);
    }
    #[test]
    /// tests S
    fn sksi_evaluates_to_i() {
        let sksi = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::K), ast::SKI::S),
            ast::SKI::I,
        );
        assert_eq!(ast::eval(sksi), ast::SKI::I);
    }

    #[test]
    fn parse_app_succeeds_with_kii() {
        assert_eq!(
            parser::parse_app(&String::from("KIS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_kii() {
        assert_eq!(
            parser::parse_ski(&String::from("KIS")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_parens_first() {
        assert_eq!(
            parser::parse_ski(&String::from("(KI)S")),
            Ok(ast::SKI::app(
                ast::SKI::app(ast::SKI::K, ast::SKI::I),
                ast::SKI::S
            ))
        );
    }
    #[test]
    fn parse_ski_succeeds_with_parens() {
        assert_eq!(
            parser::parse_ski(&String::from("K(IS)")),
            Ok(ast::SKI::app(
                ast::SKI::K,
                ast::SKI::app(ast::SKI::I, ast::SKI::S)
            ))
        );
    }
    #[test]
    fn parse_and_eval_succeeds_with_kii() {
        assert_eq!(
            parser::parse_and_eval(&String::from("KIS")),
            Ok(ast::SKI::I)
        );
    }
}
