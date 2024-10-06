use crate::binder::{
    create_child, create_empty_parent, create_optional_child, declare_symbol, AstNode, Child,
    Meaning, OptionalChild, Parent, Table,
};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::{Lexer, TokenType};
use crate::parser::try_parse_prefixed;
use crate::types::{identifier::Identifier, type_node::TypeNode};
use std::rc::Rc;

#[derive(Debug)]
pub struct Parameter {
    parent: Parent,
    name: Child<Identifier>,
    typename: OptionalChild<TypeNode>,
}

impl AstNode for Parameter {
    fn get_meaning(&self) -> Meaning {
        Meaning::Value
    }

    fn get_name(&self) -> String {
        self.name.borrow().text.clone()
    }
}

impl Parameter {
    pub fn parse(lexer: &mut Lexer) -> Result<Parameter, ParsingError> {
        let name = Identifier::parse(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(Parameter {
            parent: create_empty_parent(),
            name: create_child(name),
            typename: create_optional_child(typename),
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

        if let Some(type_node_rc) = self.typename.borrow().as_ref() {
            type_node_rc.bind(&self_rc)?;
        }

        declare_symbol(locals, &self_rc)
    }
}
