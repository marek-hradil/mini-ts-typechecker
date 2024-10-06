use crate::binder::{
    create_child, create_empty_parent, declare_symbol, AstNode, Child, Meaning, Parent, Table,
};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_expected;
use crate::types::{expression::Expression, identifier::Identifier};
use std::rc::Rc;

#[derive(Debug)]
pub struct PropertyAssignment {
    parent: Parent,
    name: Child<Identifier>,
    value: Child<Expression>,
}

impl AstNode for PropertyAssignment {
    fn get_meaning(&self) -> Meaning {
        Meaning::Value
    }

    fn get_name(&self) -> String {
        self.name.borrow().text.clone()
    }
}

impl PropertyAssignment {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyAssignment, ParsingError> {
        let name = Identifier::parse(lexer)?;
        parse_expected(lexer, TokenType::Colon)?;

        let value = Expression::parse(lexer)?;

        Ok(PropertyAssignment {
            name: create_child(name),
            value: create_child(value),
            parent: create_empty_parent(),
        })
    }

    pub fn bind(
        self: &Rc<Self>,
        parent: &Rc<dyn AstNode>,
        members: &mut Table,
    ) -> Result<(), BindingError> {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn AstNode>;
        *self.parent.borrow_mut() = Some(parent_weak);

        self.name.borrow().bind(&self_rc)?;
        self.value.borrow().bind(&self_rc)?;

        declare_symbol(members, &self_rc)?;

        Ok(())
    }
}
