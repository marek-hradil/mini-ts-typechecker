use crate::{
    errors::ParsingError,
    lexer::{Lexer, TokenType},
};

use super::{Node, Parent};

#[derive(Debug)]
pub struct Identifier {
    pub parent: Parent,
    pub text: String,
}

impl Node for Identifier {}

impl Identifier {
    pub fn parse(lexer: &mut Lexer) -> Result<Identifier, ParsingError> {
        match lexer.get() {
            Some(token) if token.token_type == TokenType::Identifier => {
                lexer.next();
                Ok(Identifier {
                    text: token.text.clone(),
                    parent: None,
                })
            }
            _ => Err(ParsingError::UnexpectedEndOfFileError),
        }
    }
}
