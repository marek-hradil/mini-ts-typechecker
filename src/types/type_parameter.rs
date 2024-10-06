use std::rc::Rc;

use super::identifier::Identifier;
use super::{create_child, create_empty_parent, Child, Node, Parent};
use crate::errors::ParsingError;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct TypeParameter {
    parent: Parent,
    name: Child<Identifier>,
}

impl Node for TypeParameter {}

impl TypeParameter {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = Identifier::parse(lexer)?;

        Ok(TypeParameter {
            parent: create_empty_parent(),
            name: create_child(name),
        })
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn Node>;
        *self.parent.borrow_mut() = Some(parent_weak);

        self.name.borrow().bind(&self_rc);
    }

    pub fn get_name(self: &Rc<Self>) -> String {
        self.name.borrow().text.clone()
    }
}
