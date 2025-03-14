use crate::parser;
use std::io;
/// defines the repl
/// runs a loop that asks user for input
/// tries to parse and evaluate input
/// prints result or error
/// exits loop when user enters quit
pub fn repl() {
    loop {
        let mut inp = String::new();
        println!("please enter a SKI expression, or enter 'quit' to exit:");
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input");
        if inp.trim() == "quit" {
            println!("goodbye!");
            break;
        }
        let parsed = parser::parse_and_eval(inp.trim());
        match parsed {
            Ok(x) => println!("{:?}", &x),
            Err(y) => println!("{:?}", &y),
        }
    }
}
