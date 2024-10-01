use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{
    parse_expected, parse_identifier, parse_sequence, try_consume_token, try_parse_prefixed,
};
use crate::types::parameter::Parameter;
use crate::types::property_assignment::PropertyAssignment;
use crate::types::statement::Statement;
use crate::types::type_node::TypeNode;
use crate::types::type_parameter::TypeParameter;
use crate::types::Location;

#[derive(Debug)]
pub struct Module {
    statements: Vec<Statement>,
}

impl Module {
    pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
        let statements = parse_sequence(
            lexer,
            Statement::parse,
            TokenType::Semicolon,
            TokenType::EOF,
        )?;

        Ok(Module { statements })
    }
}
