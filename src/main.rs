use crate::lexer::Lexer;
use crate::parser::parse;
use std::env;
use std::fs;

mod errors;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(contents) => {
            run_checker(contents);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}

pub fn run_checker(contents: String) {
    let mut lexer = Lexer::new(&contents);
    let ast = parse(&mut lexer);

    match ast {
        Ok(ast) => {
            println!("{:?}", ast);
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
