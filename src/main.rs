fn main() {
    println!("{:?}", ast::eval(ast::SKI::S));
    let app = ast::App {
        combinator: ast::SKI::I,
        arg: vec![ast::SKI::K],
    };
    println!("{:?}", ast::eval(ast::SKI::Application(Box::new(app))));
    let app2 = ast::App {
        combinator: ast::SKI::K,
        arg: vec![ast::SKI::I,ast::SKI::K],
    };
    println!("{:?}", ast::eval(ast::SKI::Application(Box::new(app2))));
}

mod ast {
    #[derive(Debug, Clone)]
    pub enum SKI {
        S,
        K,
        I,
        Application(Box<App>),
    }
    #[derive(Debug, Clone)]
    pub struct App {
        pub combinator:  SKI,
        pub arg: Vec<SKI>,
    }

    pub fn eval(skiexp: SKI) -> SKI {
        match skiexp {
            SKI::S => SKI::S,
            SKI::K => SKI::K,
            SKI::I => SKI::I,
            SKI::Application(app) => match app.arg {
                x if x.len() == 0 => app.combinator.clone(),
                _ => match app.combinator {
                    SKI::I => SKI::Application(Box::new(App {
                        combinator: app.arg[0].clone(),
                        arg: app.arg[1..].to_vec(),
                    })),
                    SKI::K => {
                        match app.arg{
                            ref x if x.len() == 1 => SKI::Application(app),
                            _ =>SKI::Application(Box::new(App {
                                combinator: app.arg[0].clone(),
                                arg: app.arg[2..].to_vec(),
                            })),
                        }
                    }
                    _ => SKI::S,
                },
            },
        }
    }
}
