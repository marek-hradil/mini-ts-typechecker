use super::identifier::Identifier;
use super::Node;
use super::{type_node::TypeNode, Location};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_identifier, try_parse_prefixed};

#[derive(Debug)]
pub struct Parameter {
    location: Location,
    name: Identifier,
    typename: Option<TypeNode>,
}

impl Node for Parameter {}

impl Parameter {
    pub fn parse(lexer: &mut Lexer) -> Result<Parameter, ParsingError> {
        let text = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(Parameter {
            location: Default::default(),
            name: Identifier {
                text: text,
                location: Default::default(),
            },
            typename: typename,
        })
    }
}
