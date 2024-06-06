
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
mod tests {
    use super::*;
    #[test]
    /// tests that i is irreducable
    fn i_evaluates_to_i() {
        let result = eval(SKI::I);
        assert_eq!(result, SKI::I);
    }
    #[test]
    /// tests that K is irreducable
    fn k_evaluates_to_k() {
        let result = eval(SKI::K);
        assert_eq!(result, SKI::K);
    }
    #[test]
    /// tests that II reduces to I
    fn ii_evaluates_to_i() {
        let ii = SKI::app(SKI::I, SKI::I);
        assert_eq!(eval(ii), SKI::I);
    }
    #[test]
    /// tests III reduces to I
    fn iii_evaluates_to_i() {
        let ii = SKI::app(SKI::I, SKI::I);
        let iii = SKI::app(ii, SKI::I);
        assert_eq!(eval(iii), SKI::I);
    }
    #[test]
    fn iik_evaluates_to_k() {
        let ii = SKI::app(SKI::I, SKI::I);
        let iik = SKI::app(ii, SKI::K);
        assert_eq!(eval(iik), SKI::K);
    }
    #[test]
    /// tests that K returns the first argument
    fn kik_evaluates_to_i() {
        let ki = SKI::app(SKI::K, SKI::I);
        let kik = SKI::app(ki, SKI::K);
        assert_eq!(eval(kik), SKI::I);
    }
    #[test]
    /// tests that KI(KI) reduces to I
    fn kiki_evaluates_to_i() {
        let ki = SKI::app(SKI::K, SKI::I);
        let kiki = SKI::app(ki.clone(), ki.clone());
        assert_eq!(eval(kiki), SKI::I);
    }
    #[test]
    /// tests that KIKS reduces to S
    fn kiks_evaluates_to_s() {
        let kiks = SKI::app(SKI::app(SKI::app(SKI::K, SKI::I), SKI::K), SKI::S);
        assert_eq!(eval(kiks), SKI::S);
    }
    #[test]
    /// tests S
    fn sksi_evaluates_to_i() {
        let sksi = SKI::app(SKI::app(SKI::app(SKI::S, SKI::K), SKI::S), SKI::I);
        assert_eq!(eval(sksi), SKI::I);
    }
}
