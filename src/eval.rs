#[cfg(test)]
extern crate quickcheck;
extern crate quickcheck_macros;
use crate::ast;
/// function eval reduces a ast::SKI expression to a simpler one if reducable
pub fn eval(skiexp: ast::SKI) -> ast::SKI {
    match skiexp {
        ast::SKI::Application(boxed_app) => {
            let app = boxed_app;
            match &app.combinator {
                ast::SKI::I => eval(app.arg.clone()),

                ast::SKI::Application(boxed_app2) => {
                    let app2 = boxed_app2;
                    match &app2.combinator {
                        ast::SKI::K => eval(app2.arg.clone()),
                        ast::SKI::Application(boxed_app3) => {
                            let app3 = boxed_app3;
                            match &app3.combinator {
                                ast::SKI::S => eval(ast::SKI::app(
                                    ast::SKI::app(app3.arg.clone(), app.arg.clone()),
                                    ast::SKI::app(app2.arg.clone(), app3.arg.clone()),
                                )),

                                _ => eval(ast::SKI::app(
                                    eval(app.combinator.clone()),
                                    app.arg.clone(),
                                )),
                            }
                        }

                        ast::SKI::S => ast::SKI::app(
                            ast::SKI::app(ast::SKI::S, eval(app2.arg.clone())),
                            eval(app.arg.clone()),
                        ),
                        _ => eval(ast::SKI::app(eval(app.combinator.clone()), app.arg.clone())),
                    }
                }

                ast::SKI::K => ast::SKI::app(ast::SKI::K, eval(app.arg.clone())),

                ast::SKI::S => ast::SKI::app(ast::SKI::S, eval(app.arg.clone())),
            }
        }

        ski => ski,
    }
}
fn reverse<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut rev = vec![];
    for x in xs.iter() {
        rev.insert(0, x.clone())
    }
    rev
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[quickcheck_macros::quickcheck]
    /// A property that I is identity
    fn prop_I_identity(ski: ast::SKI) -> bool {
        let i_applied_to_ski =ast::SKI::app(ast::SKI::I, ski.clone());
        eval(ski) == eval(i_applied_to_ski)
    }
    #[test]
    /// tests that i is irreducable
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
