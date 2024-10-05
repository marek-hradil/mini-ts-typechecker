use super::expression::Expression;
use super::identifier::Identifier;
use super::{Node, Parent};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_expected;

#[derive(Debug)]
pub struct PropertyAssignment {
    parent: Parent,
    name: Identifier,
    value: Expression,
}

impl Node for PropertyAssignment {}

impl PropertyAssignment {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyAssignment, ParsingError> {
        let name = Identifier::parse(lexer)?;
        parse_expected(lexer, TokenType::Colon)?;

        let value = Expression::parse(lexer)?;

        Ok(PropertyAssignment {
            name,
            value,
            parent: None,
        })
    }
}
