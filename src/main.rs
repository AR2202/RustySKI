use RustySKI::ast;
use RustySKI::parser;
use RustySKI::eval;
fn main() {
    println!("{:?}", eval::eval(ast::SKI::S));
    let app = ast::App {
        combinator: ast::SKI::I,
        arg: ast::SKI::K,
    };
    println!("{:?}", eval::eval(ast::SKI::Application(Box::new(app))));
    println!(
        "{:?}",
        parser::parse_and_eval("KI(IS)").unwrap_or(ast::SKI::K)
    );
}
