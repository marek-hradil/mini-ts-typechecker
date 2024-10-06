use std::rc::Rc;

use super::identifier::Identifier;
use super::type_node::TypeNode;
use super::{
    create_child, create_empty_parent, create_optional_child, Child, Node, OptionalChild, Parent,
};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::try_parse_prefixed;

#[derive(Debug)]
pub struct PropertyDeclaration {
    parent: Parent,
    name: Child<Identifier>,
    typename: OptionalChild<TypeNode>,
}

impl Node for PropertyDeclaration {}

impl PropertyDeclaration {
    pub fn parse(lexer: &mut Lexer) -> Result<PropertyDeclaration, ParsingError> {
        let name = Identifier::parse(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(PropertyDeclaration {
            name: create_child(name),
            typename: create_optional_child(typename),
            parent: create_empty_parent(),
        })
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn Node>;
        *self.parent.borrow_mut() = Some(parent_weak);

        self.name.borrow().bind(&self_rc);

        if let Some(type_node_rc) = self.typename.borrow().as_ref() {
            type_node_rc.bind(&self_rc);
        }
    }

    pub fn get_name(self: &Rc<Self>) -> String {
        self.name.borrow().text.clone()
    }
}
