use std::fmt;

#[derive(Clone, PartialEq)]
pub enum SKI {
    S,
    K,
    I,
    Application(Box<App>),
}
#[derive(Clone, PartialEq)]
pub struct App {
    pub combinator: SKI,
    pub arg: SKI,
}
/// custom formatter for SKI
impl fmt::Debug for SKI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SKI::I => write!(f, "I"),
            SKI::K => write!(f, "K"),
            SKI::S => write!(f, "S"),
            SKI::Application(x) => write!(f, "{:?}", x),
        }
    }
}
/// custom formatter for App
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.combinator, self.arg)
    }
}
impl SKI {
    pub fn is_application(&self) -> bool {
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
//pub type SKIErr = String;
#[derive(Debug, PartialEq, Clone)]
pub enum SKIErr {
    ParseError(String),
    SyntaxError(String),
}
