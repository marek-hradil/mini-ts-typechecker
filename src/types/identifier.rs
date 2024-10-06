use std::rc::Rc;

use crate::{
    errors::ParsingError,
    lexer::{Lexer, TokenType},
};

use super::{create_empty_parent, Node, Parent};

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
                    parent: create_empty_parent(),
                })
            }
            _ => Err(ParsingError::UnexpectedEndOfFileError),
        }
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        *self.parent.borrow_mut() = Some(parent_weak);
    }
}
