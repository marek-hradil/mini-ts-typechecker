use crate::binder::bind;
use crate::lexer::Lexer;
use crate::parser::parse;
use std::env;
use std::fs;

mod binder;
mod errors;
mod lexer;
mod parser;
mod types;

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
    let mut lexer = Lexer::new(contents.leak());
    let ast = parse(&mut lexer);
    let binded_ast = bind(ast.unwrap()); // @todo: handle unwrap

    println!("{:?}", binded_ast);
}
