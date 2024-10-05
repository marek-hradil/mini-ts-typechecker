use super::identifier::Identifier;
use super::type_node::TypeNode;
use super::{Node, Parent};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::try_parse_prefixed;

#[derive(Debug)]
pub struct PropertyDeclaration {
    parent: Parent,
    name: Identifier,
    typename: Option<TypeNode>,
}

impl Node for PropertyDeclaration {}

impl PropertyDeclaration {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyDeclaration, ParsingError> {
        let name = Identifier::parse(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(PropertyDeclaration {
            name,
            typename,
            parent: None,
        })
    }
}
