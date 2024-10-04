use RustySKI::ast;
use RustySKI::parser;
use RustySKI::eval;
use RustySKI::repl;
fn main() {
    // this prints SKI on separate lines
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
    // calling repl
    repl::repl();
}
