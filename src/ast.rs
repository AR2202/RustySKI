use quickcheck::{Arbitrary, Gen};
use std::fmt;
use std::iter;
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
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            SKI::S | SKI::K | SKI::I => Box::new(iter::empty()),
            SKI::Application(app) => {
                let shrinked_app = app.shrink().map(SKI::Application);
                let simple_forms = vec![SKI::S, SKI::K, SKI::I];
                Box::new(simple_forms.into_iter().chain(shrinked_app))
            }
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

    fn shrink(&self) -> Box<dyn Iterator<Item = App>> {
        let mut shrinks = Vec::new();

        // Shrink `combinator` only, keeping `arg` the same
        for c in self.combinator.shrink() {
            shrinks.push(App {
                combinator: c,
                arg: self.arg.clone(),
            });
        }

        // Shrink `arg` only, keeping `combinator` the same
        for a in self.arg.shrink() {
            shrinks.push(App {
                combinator: self.combinator.clone(),
                arg: a,
            });
        }

        Box::new(shrinks.into_iter())
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
        matches!(self, SKI::Application(_))
    }
    /// simple helper function for creating the Application variant of the SKI enum
    pub fn app(combinator: SKI, arg: SKI) -> SKI {
        SKI::Application(Box::new(App { combinator, arg }))
    }
}

#[derive(PartialEq, Clone)]
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
