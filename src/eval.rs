#[cfg(test)]
extern crate quickcheck;
extern crate quickcheck_macros;
use crate::ast;
/// function eval reduces a ast::SKI expression to a simpler one if reducable
pub fn eval(skiexp: ast::SKI) -> ast::SKI {
    match skiexp {
        //check if it's the Application variant of the enum
        ast::SKI::Application(app) => match &app.combinator {
            //I can be evaluated with only 1 argument
            ast::SKI::I => eval(app.arg),
            ast::SKI::Application(app2) => match &app2.combinator {
                //if 2 arguments are given, K returns the first
                ast::SKI::K => eval(app2.arg.clone()),
                //S can be evaluated with 3 arguments
                //applying the first to the third
                //the second to the third 
                //and the result of the first application to the second
                ast::SKI::Application(app3) => match &app3.combinator {
                    ast::SKI::S => eval(ast::SKI::app(
                        ast::SKI::app(app3.arg.clone(), app.arg.clone()),
                        ast::SKI::app(app2.arg.clone(), app.arg.clone()),
                    )),

                    _ => eval(ast::SKI::app(eval(app.combinator.clone()), app.arg.clone())),
                },

                ast::SKI::S => ast::SKI::app(
                    ast::SKI::app(ast::SKI::S, eval(app2.arg.clone())),
                    eval(app.arg.clone()),
                ),
                _ => eval(ast::SKI::app(eval(app.combinator.clone()), app.arg.clone())),
            },
            
            // the K variant needs 2 arguments
            // if only 1 is given, evaluate the argument and apply K to it
            ast::SKI::K => ast::SKI::app(ast::SKI::K, eval(app.arg.clone())),
            // the S variant needs 3 arguments
            // if only 1 is given, evaluate the argument
            // and apply S to it 
            ast::SKI::S => ast::SKI::app(ast::SKI::S, eval(app.arg.clone())),
        },
        //in case it's a ski primitive, it is returned without modificaiton
        ski => ski,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck_macros::quickcheck]
    /// A property that I is identity
    fn prop_i_identity(ski: ast::SKI) -> bool {
        let i_applied_to_ski = ast::SKI::app(ast::SKI::I, ski.clone());
        eval(ski) == eval(i_applied_to_ski)
    }
    #[quickcheck_macros::quickcheck]
    /// A property that SKx is equivalent to I
    fn prop_skx_identity(ski: ast::SKI, ski2: ast::SKI) -> bool {
        let skx_applied_to_ski = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::K), ski2),
            ski.clone(),
        );
        eval(ski) == eval(skx_applied_to_ski)
    }
    #[quickcheck_macros::quickcheck]
    /// A property that K returns the first argument
    fn prop_k_returns_first_arg(arg1: ast::SKI, arg2: ast::SKI) -> bool {
        let k_applied_to_both = ast::SKI::app(ast::SKI::app(ast::SKI::K, arg1.clone()), arg2);
        eval(arg1) == eval(k_applied_to_both)
    }
    #[test]
    /// tests that I is irreducable
    fn i_evaluates_to_i() {
        let result = eval(ast::SKI::I);
        assert_eq!(result, ast::SKI::I);
    }
    #[test]
    /// tests that K is irreducable
    fn k_evaluates_to_k() {
        let result = eval(ast::SKI::K);
        assert_eq!(result, ast::SKI::K);
    }
    #[test]
    /// tests that II reduces to I
    fn ii_evaluates_to_i() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        assert_eq!(eval(ii), ast::SKI::I);
    }
    #[test]
    /// tests III reduces to I
    fn iii_evaluates_to_i() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        let iii = ast::SKI::app(ii, ast::SKI::I);
        assert_eq!(eval(iii), ast::SKI::I);
    }
    #[test]
    fn iik_evaluates_to_k() {
        let ii = ast::SKI::app(ast::SKI::I, ast::SKI::I);
        let iik = ast::SKI::app(ii, ast::SKI::K);
        assert_eq!(eval(iik), ast::SKI::K);
    }
    #[test]
    /// tests that K returns the first argument
    fn kik_evaluates_to_i() {
        let ki = ast::SKI::app(ast::SKI::K, ast::SKI::I);
        let kik = ast::SKI::app(ki, ast::SKI::K);
        assert_eq!(eval(kik), ast::SKI::I);
    }
    #[test]
    /// tests that KI(KI) reduces to I
    fn kiki_evaluates_to_i() {
        let ki = ast::SKI::app(ast::SKI::K, ast::SKI::I);
        let kiki = ast::SKI::app(ki.clone(), ki.clone());
        assert_eq!(eval(kiki), ast::SKI::I);
    }
    #[test]
    /// tests that KIKS reduces to S
    fn kiks_evaluates_to_s() {
        let kiks = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::K, ast::SKI::I), ast::SKI::K),
            ast::SKI::S,
        );
        assert_eq!(eval(kiks), ast::SKI::S);
    }

    #[test]
    /// tests SKSI reduces to I
    fn sksi_evaluates_to_i() {
        let sksi = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::K), ast::SKI::S),
            ast::SKI::I,
        );
        assert_eq!(eval(sksi), ast::SKI::I);
    }
    #[test]
    /// tests SKSI reduces to I
    fn s_with_parens() {
        let sksi_parens = ast::SKI::app(
            ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::K), ast::SKI::S),
            ast::SKI::I,
        );
        assert_eq!(eval(sksi_parens), ast::SKI::I);
    }
    #[test]
    /// tests SKSI reduces to I
    fn k_with_parens() {
        let kiisk_parens = ast::SKI::app(ast::SKI::app(ast::SKI::app(ast::SKI::K, ast::SKI::app(ast::SKI::I,ast::SKI::I)), ast::SKI::S),
            ast::SKI::K,
        );
        assert_eq!(eval(kiisk_parens), ast::SKI::K);
    }
    #[test]
    /// tests SKSI reduces to I
    fn sk_is_irreducable() {
        let sk = ast::SKI::app(ast::SKI::S, ast::SKI::K);
        assert_eq!(eval(sk.clone()), sk);
    }
    #[test]
    /// tests SKSI reduces to I
    fn ski_is_irreducable() {
        let ski = ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::K), ast::SKI::I);
        assert_eq!(eval(ski.clone()), ski);
    }
    #[test]
    /// tests SKSI reduces to I
    fn sik_is_irreducable() {
        let sik = ast::SKI::app(ast::SKI::app(ast::SKI::S, ast::SKI::I), ast::SKI::K);
        assert_eq!(eval(sik.clone()), sik);
    }
}
