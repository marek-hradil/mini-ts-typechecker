use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_identifier, try_parse_prefixed};
use crate::types::type_node::TypeNode;
use crate::types::Location;

#[derive(Debug)]
pub struct PropertyDeclaration {
    location: Location,
    name: String,
    typename: Option<TypeNode>,
}

impl PropertyDeclaration {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyDeclaration, ParsingError> {
        let name = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(PropertyDeclaration {
            name,
            typename,
            location: Default::default(),
        })
    }
}
