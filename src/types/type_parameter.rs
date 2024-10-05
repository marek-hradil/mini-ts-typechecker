use super::identifier::Identifier;
use super::{Location, Node};
use crate::errors::ParsingError;
use crate::lexer::Lexer;
use crate::parser::parse_identifier;

#[derive(Debug)]
pub struct TypeParameter {
    location: Location,
    name: Identifier,
}

impl Node for TypeParameter {}

impl TypeParameter {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = parse_identifier(lexer)?;

        Ok(TypeParameter {
            name: Identifier {
                text: name,
                location: Default::default(),
            },
            location: Default::default(),
        })
    }
}
