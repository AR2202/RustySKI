fn main() {
    println!("{:?}", ast::eval(ast::SKI::S));
    let app = ast::App {
        combinator: ast::SKI::I,
        arg: ast::SKI::K,
    };
    println!("{:?}", ast::eval(ast::SKI::Application(Box::new(app))));
    let app2 = ast::App {
        combinator: ast::SKI::K,
        arg: ast::SKI::I,
    };
    println!("{:?}", ast::eval(ast::SKI::Application(Box::new(app2))));
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
    use crate::ast;
    /// parses I combinator at the beginning of String and returns a tuple of the parsed combinator and the rest of the string; if it fails, returns a SKIErr
    pub fn parse_i(inp: &str) -> Result<(ast::SKI, String), ast::SKIErr> {
        let mut characters = inp.chars();
        match characters.next().unwrap() {
            'I' => Ok((ast::SKI::I, characters.collect())),
            _ => Err(String::from("not I")),
        }
    }
    pub fn parse_k(inp: &str) -> Result<(ast::SKI, String), ast::SKIErr> {
        let mut characters = inp.chars();
        match characters.next().unwrap() {
            'K' => Ok((ast::SKI::K, characters.collect())),
            _ => Err(String::from("not K")),
        }
    }
    pub fn parse_s(inp: &str) -> Result<(ast::SKI, String), ast::SKIErr> {
        let mut characters = inp.chars();
        match characters.next().unwrap() {
            'S' => Ok((ast::SKI::S, characters.collect())),
            _ => Err(String::from("not S")),
        }
    }
    pub fn parse_primitive(inp: &str) -> Result<(ast::SKI, String), ast::SKIErr> {
        match parse_i(inp) {
            Ok(parsed) => Ok(parsed),
            Err(_) => match parse_k(inp) {
                Ok(parsed) => Ok(parsed),
                Err(_) => match parse_s(inp) {
                    Ok(parsed) => Ok(parsed),
                    Err(_) => Err(String::from("no SKI primitive")),
                },
            },
        }
    }
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
        if inp.ends_with(')') {
            for (i, c) in inp.char_indices() {
                if c == '(' {
                    return Ok(ast::SKI::app(
                        parse_ski(&inp[..i])?,
                        parse_ski(&inp[i + 1..inp.len() - 1])?,
                    ));
                }
            }
            return Err(String::from("unclosed parentheses"));
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
    /// tests parse
    fn parse_i_succeeds_with_i() {
        assert_eq!(
            parser::parse_i(&String::from("IK")),
            Ok((ast::SKI::I, String::from("K")))
        );
    }
    #[test]
    fn parse_i_fails_with_k() {
        assert_eq!(
            parser::parse_i(&String::from("KIK")),
            Err(String::from("not I"))
        );
    }
    #[test]
    fn parse_primitive_with_k() {
        assert_eq!(
            parser::parse_primitive(&String::from("KIK")),
            Ok((ast::SKI::K, String::from("IK")))
        );
    }
    #[test]
    fn parse_primitive_fails_with_t() {
        assert_eq!(
            parser::parse_primitive(&String::from("TIK")),
            Err(String::from("no SKI primitive"))
        );
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
            Ok(
                ast::SKI::app(ast::SKI::K, ast::SKI::app(ast::SKI::I,
                ast::SKI::S)
            ))
        );
    }
}
