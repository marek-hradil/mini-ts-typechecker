use super::identifier::Identifier;
use super::{Node, Parent};
use crate::errors::ParsingError;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct TypeParameter {
    parent: Parent,
    name: Identifier,
}

impl Node for TypeParameter {}

impl TypeParameter {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = Identifier::parse(lexer)?;

        Ok(TypeParameter { parent: None, name })
    }
}
