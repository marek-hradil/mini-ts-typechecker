use std::rc::Rc;

use super::{create_child, create_children, create_empty_parent, Child, Children, Parent};
use super::{
    identifier::Identifier, parameter::Parameter, property_declaration::PropertyDeclaration,
    type_parameter::TypeParameter, Node,
};
use crate::binder::Table;
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_sequence, try_consume_token};

#[derive(Debug)]
pub enum TypeNode {
    ObjectLiteralType {
        parent: Parent,
        properties: Children<PropertyDeclaration>,
    },
    Identifier(Child<Identifier>),
    SignatureDeclaration {
        parent: Parent,
        type_parameters: Children<TypeParameter>,
        parameters: Children<Parameter>,
        typename: Child<TypeNode>,
        locals: Table,
    },
}

impl Node for TypeNode {}

impl TypeNode {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeNode, ParsingError> {
        if try_consume_token(lexer, &TokenType::OpenBrace) {
            let properties = parse_sequence(
                lexer,
                PropertyDeclaration::parse,
                TokenType::Comma,
                TokenType::CloseBrace,
            )?;

            Ok(TypeNode::ObjectLiteralType {
                parent: create_empty_parent(),
                properties: create_children(properties),
            })
        } else if try_consume_token(lexer, &TokenType::LessThan) {
            let type_parameters = parse_sequence(
                lexer,
                TypeParameter::parse,
                TokenType::Comma,
                TokenType::GreaterThan,
            )?;

            parse_expected(lexer, TokenType::OpenParen)?;

            let parameters = parse_sequence(
                lexer,
                Parameter::parse,
                TokenType::Comma,
                TokenType::CloseParen,
            )?;

            parse_expected(lexer, TokenType::Arrow)?;

            let typename = TypeNode::parse(lexer)?;

            Ok(TypeNode::SignatureDeclaration {
                parent: create_empty_parent(),
                type_parameters: create_children(type_parameters),
                parameters: create_children(parameters),
                typename: create_child(typename),
                locals: Table::new(),
            })
        } else if try_consume_token(lexer, &TokenType::OpenParen) {
            let parameters = parse_sequence(
                lexer,
                Parameter::parse,
                TokenType::Comma,
                TokenType::CloseParen,
            )?;

            parse_expected(lexer, TokenType::Arrow)?;

            let typename = TypeNode::parse(lexer)?;

            Ok(TypeNode::SignatureDeclaration {
                parent: create_empty_parent(),
                type_parameters: create_children(vec![]),
                parameters: create_children(parameters),
                typename: create_child(typename),
                locals: Table::new(),
            })
        } else {
            Ok(TypeNode::Identifier(create_child(Identifier::parse(
                lexer,
            )?)))
        }
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn Node>;

        match &**self {
            TypeNode::ObjectLiteralType { parent, properties } => {
                *parent.borrow_mut() = Some(parent_weak);
                properties
                    .borrow()
                    .iter()
                    .for_each(|property| property.bind(&self_rc));
            }
            TypeNode::Identifier(identifier) => {
                identifier.borrow().bind(&self_rc);
            }
            TypeNode::SignatureDeclaration {
                parent,
                type_parameters,
                parameters,
                typename,
                ..
            } => {
                *parent.borrow_mut() = Some(parent_weak);
                type_parameters
                    .borrow()
                    .iter()
                    .for_each(|type_parameter| type_parameter.bind(&self_rc));
                parameters
                    .borrow()
                    .iter()
                    .for_each(|parameter| parameter.bind(&self_rc));
                typename.borrow().bind(&self_rc);
            }
        }
    }
}
