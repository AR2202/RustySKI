use std::fmt;
use quickcheck::{Arbitrary, Gen};
#[derive(Clone, PartialEq)]
pub enum SKI {
    S,
    K,
    I,
    Application(Box<App>),
}
impl Arbitrary for SKI {
    fn arbitrary(g: &mut Gen) -> SKI {
        // Define the likelihood for each variant
        let choice = u8::arbitrary(g) % 4;
        match choice {
            0 => SKI::S,
            1 => SKI::K,
            2 => SKI::I,
            _ => SKI::Application(Box::new(App::arbitrary(g))),
        }
    }
}
impl Arbitrary for App {
    fn arbitrary(g: &mut Gen) -> App {
        App {
            combinator: SKI::arbitrary(g),
            arg: SKI::arbitrary(g),
        }
    }
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
        match &self.arg {
            SKI::Application(x) => write!(f, "{:?}({:?})", self.combinator, self.arg),
            _ => write!(f, "{:?}{:?}", self.combinator, self.arg),
        }
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

#[derive( PartialEq, Clone)]
pub enum SKIErr {
    ParseError(String),
    SyntaxError(String),
}
impl fmt::Debug for SKIErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SKIErr::ParseError(x) => write!(f, "Parse Error: {}", x),
            SKIErr::SyntaxError(x) => write!(f, "Syntax Error: {}", x),
        }
    }
}
