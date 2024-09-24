use crate::lex::Lexer;
use std::env;
use std::fs;

mod errors;
mod lex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(contents) => {
            let lexer = Lexer::new(&contents);

            for token in lexer {
                println!("{:?}", token);
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
