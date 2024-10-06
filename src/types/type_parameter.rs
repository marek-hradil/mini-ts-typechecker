use crate::binder::{
    create_child, create_empty_parent, declare_symbol, AstNode, Child, Meaning, Parent, Table,
};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::Lexer;
use crate::types::identifier::Identifier;
use std::rc::Rc;

#[derive(Debug)]
pub struct TypeParameter {
    parent: Parent,
    name: Child<Identifier>,
}

impl AstNode for TypeParameter {
    fn get_meaning(&self) -> Meaning {
        Meaning::Type
    }

    fn get_name(&self) -> String {
        self.name.borrow().text.clone()
    }
}

impl TypeParameter {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = Identifier::parse(lexer)?;

        Ok(TypeParameter {
            parent: create_empty_parent(),
            name: create_child(name),
        })
    }

    pub fn bind(
        self: &Rc<Self>,
        parent: &Rc<dyn AstNode>,
        locals: &mut Table,
    ) -> Result<(), BindingError> {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn AstNode>;
        *self.parent.borrow_mut() = Some(parent_weak);

        self.name.borrow().bind(&self_rc)?;

        declare_symbol(locals, &self_rc)?;

        Ok(())
    }
}
