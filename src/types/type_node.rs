use crate::binder::{
    create_child, create_children, create_empty_parent, AstNode, Child, Children, Meaning, Parent,
    Table,
};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_sequence, try_consume_token};
use crate::types::{
    identifier::Identifier, parameter::Parameter, property_declaration::PropertyDeclaration,
    type_parameter::TypeParameter,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum TypeNode {
    ObjectLiteralType {
        parent: Parent,
        properties: Children<PropertyDeclaration>,
        members: RefCell<Table>,
    },
    Identifier(Child<Identifier>),
    SignatureDeclaration {
        parent: Parent,
        type_parameters: Children<TypeParameter>,
        parameters: Children<Parameter>,
        typename: Child<TypeNode>,
        locals: RefCell<Table>,
    },
}

impl AstNode for TypeNode {
    fn get_meaning(&self) -> Meaning {
        Meaning::Type
    }

    fn get_name(&self) -> String {
        match self {
            TypeNode::ObjectLiteralType { .. } => String::from("__object"),
            TypeNode::Identifier(identifier) => identifier.borrow().text.clone(),
            TypeNode::SignatureDeclaration { .. } => String::from("__signature"),
        }
    }
}

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
                members: RefCell::new(Table::new()),
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
                locals: RefCell::new(Table::new()),
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
                locals: RefCell::new(Table::new()),
            })
        } else {
            Ok(TypeNode::Identifier(create_child(Identifier::parse(
                lexer,
            )?)))
        }
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn AstNode>) -> Result<(), BindingError> {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn AstNode>;

        match &**self {
            TypeNode::ObjectLiteralType {
                parent,
                properties,
                members,
            } => {
                *parent.borrow_mut() = Some(parent_weak);

                for property in properties.borrow().iter() {
                    property.bind(&self_rc, &mut members.borrow_mut())?;
                }

                Ok(())
            }
            TypeNode::Identifier(identifier) => {
                identifier.borrow().bind(&self_rc)?;

                Ok(())
            }
            TypeNode::SignatureDeclaration {
                parent,
                type_parameters,
                parameters,
                typename,
                locals,
            } => {
                *parent.borrow_mut() = Some(parent_weak);

                for type_parameter in type_parameters.borrow().iter() {
                    type_parameter.bind(&self_rc, &mut locals.borrow_mut())?;
                }

                for parameter in parameters.borrow().iter() {
                    parameter.bind(&self_rc, &mut locals.borrow_mut())?;
                }

                typename.borrow().bind(&self_rc)?;

                Ok(())
            }
        }
    }
}
