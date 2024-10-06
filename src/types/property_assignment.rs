use std::rc::Rc;

use super::expression::Expression;
use super::identifier::Identifier;
use super::{create_child, create_empty_parent, Child, Node, Parent};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_expected;

#[derive(Debug)]
pub struct PropertyAssignment {
    parent: Parent,
    name: Child<Identifier>,
    value: Child<Expression>,
}

impl Node for PropertyAssignment {}

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

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn Node>;
        *self.parent.borrow_mut() = Some(parent_weak);

        self.name.borrow().bind(&self_rc);
        self.value.borrow().bind(&self_rc);
    }

    pub fn get_name(self: &Rc<Self>) -> String {
        self.name.borrow().text.clone()
    }
}
