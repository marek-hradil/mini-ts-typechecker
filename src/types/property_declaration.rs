use super::identifier::Identifier;
use super::Node;
use super::{type_node::TypeNode, Location};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_identifier, try_parse_prefixed};

#[derive(Debug)]
pub struct PropertyDeclaration {
    location: Location,
    name: Identifier,
    typename: Option<TypeNode>,
}

impl Node for PropertyDeclaration {}

impl PropertyDeclaration {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyDeclaration, ParsingError> {
        let text = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(PropertyDeclaration {
            name: Identifier {
                text: text,
                location: Default::default(),
            },
            typename,
            location: Default::default(),
        })
    }
}
