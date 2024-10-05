use super::identifier::Identifier;
use super::Node;
use super::{expression::Expression, Location};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_identifier};

#[derive(Debug)]
pub struct PropertyAssignment {
    location: Location,
    name: Identifier,
    value: Expression,
}

impl Node for PropertyAssignment {}

impl PropertyAssignment {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyAssignment, ParsingError> {
        let text = parse_identifier(lexer)?;
        parse_expected(lexer, TokenType::Colon)?;

        let value = Expression::parse(lexer)?;

        Ok(PropertyAssignment {
            name: Identifier {
                text: text,
                location: Default::default(),
            },
            value,
            location: Default::default(),
        })
    }
}
