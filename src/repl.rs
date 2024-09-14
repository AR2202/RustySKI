use crate::parser;
use std::io;
pub fn repl() {
    loop {
        let mut inp = String::new();

        println!("please enter a SKI expression, or enter 'quit' to exit:");
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read line");
        if inp.trim() == String::from("quit") {
            println!("goodbye!");
            break;
        }

        let parsed = parser::parse_and_eval(&inp.trim());
        match parsed{
            Ok(x) => println!("{:?}", x),
            Err(y) => println!("{:?}",&y),
        }

    }
}
