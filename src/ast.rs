
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
pub type SKIErr = String;
