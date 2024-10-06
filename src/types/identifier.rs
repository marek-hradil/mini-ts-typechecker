use crate::{
    binder::{create_empty_parent, AstNode, Meaning, Parent},
    errors::{BindingError, ParsingError},
    lexer::{Lexer, TokenType},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct Identifier {
    pub parent: Parent,
    pub text: String,
}

impl AstNode for Identifier {
    fn get_meaning(&self) -> Meaning {
        Meaning::Value
    }

    fn get_name(&self) -> String {
        self.text.clone()
    }
}

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

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn AstNode>) -> Result<(), BindingError> {
        let parent_weak = Rc::downgrade(parent);
        *self.parent.borrow_mut() = Some(parent_weak);

        Ok(())
    }
}
