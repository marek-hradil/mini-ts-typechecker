use super::{statement::Statement, Node};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_sequence;

#[derive(Debug)]
pub struct Module {
    statements: Vec<Statement>,
}

impl Node for Module {}

impl Module {
    pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
        let statements = parse_sequence(
            lexer,
            Statement::parse,
            TokenType::Semicolon,
            TokenType::EOF,
        )?;

        let module = Module {
            statements: statements,
        };

        Ok(module)
    }
}
