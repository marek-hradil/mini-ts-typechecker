use super::{expression::Expression, Location};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_identifier};

#[derive(Debug)]
pub struct PropertyAssignment {
    location: Location,
    name: String,
    value: Expression,
}

impl PropertyAssignment {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyAssignment, ParsingError> {
        let name = parse_identifier(lexer)?;
        parse_expected(lexer, TokenType::Colon)?;

        let value = Expression::parse(lexer)?;

        Ok(PropertyAssignment {
            name,
            value,
            location: Default::default(),
        })
    }
}
