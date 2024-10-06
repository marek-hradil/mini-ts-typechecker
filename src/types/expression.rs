use core::panic;
use std::rc::Rc;

use super::{
    create_child, create_children, create_empty_parent, create_optional_child, Child, Children,
    OptionalChild, Parent,
};
use super::{
    identifier::Identifier, parameter::Parameter, property_assignment::PropertyAssignment,
    statement::Statement, type_node::TypeNode, type_parameter::TypeParameter, Node,
};
use crate::binder::Table;
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_sequence, try_consume_token, try_parse_prefixed};

#[derive(Debug)]
pub enum Expression {
    Identifier(Child<Identifier>),
    NumericLiteral {
        value: i64,
    },
    StringLiteral {
        value: String,
    },
    Assignment {
        parent: Parent,
        name: Child<Identifier>,
        value: Child<Expression>,
    },
    Object {
        parent: Parent,
        properties: Children<PropertyAssignment>,
    },
    Function {
        parent: Parent,
        name: OptionalChild<Identifier>,
        type_parameters: Children<TypeParameter>,
        parameters: Children<Parameter>,
        typename: OptionalChild<TypeNode>,
        body: Children<Statement>,
        locals: Table,
    },
    Call {
        parent: Parent,
        expression: Child<Expression>,
        type_arguments: Children<TypeNode>,
        arguments: Children<Expression>,
    },
}

impl Node for Expression {}
impl Expression {
    pub fn parse(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
        let expression = Expression::parse_below_call(lexer)?;

        let type_arguments = if try_consume_token(lexer, &TokenType::LessThan) {
            parse_sequence(
                lexer,
                TypeNode::parse,
                TokenType::Comma,
                TokenType::GreaterThan,
            )?
        } else {
            vec![]
        };

        if try_consume_token(lexer, &TokenType::OpenParen) {
            let arguments = parse_sequence(
                lexer,
                Expression::parse,
                TokenType::Comma,
                TokenType::CloseParen,
            )?;

            Ok(Expression::Call {
                parent: create_empty_parent(),
                expression: create_child(expression),
                type_arguments: create_children(type_arguments),
                arguments: create_children(arguments),
            })
        } else {
            Ok(expression)
        }
    }

    pub fn bind(self: &Rc<Self>, parent: &Rc<dyn Node>) {
        let parent_weak = Rc::downgrade(parent);
        let self_rc = Rc::clone(self) as Rc<dyn Node>;

        match &**self {
            Expression::Identifier(name) => {
                name.borrow().bind(&self_rc);
            }
            Expression::NumericLiteral { .. } => {}
            Expression::StringLiteral { .. } => {}
            Expression::Assignment {
                name,
                value,
                parent,
            } => {
                *parent.borrow_mut() = Some(parent_weak);

                name.borrow().bind(&self_rc);
                value.borrow().bind(&self_rc);
            }
            Expression::Object { properties, parent } => {
                *parent.borrow_mut() = Some(parent_weak);

                for property in properties.borrow().iter() {
                    property.bind(&self_rc);
                }
            }
            Expression::Function {
                name,
                type_parameters,
                parameters,
                typename,
                body,
                parent,
                locals,
            } => {
                *parent.borrow_mut() = Some(parent_weak);

                if let Some(name_rc) = name.borrow().as_ref() {
                    name_rc.bind(&self_rc);
                }

                if let Some(typename_rc) = typename.borrow().as_ref() {
                    typename_rc.bind(&self_rc);
                }

                for type_parameter in type_parameters.borrow().iter() {
                    type_parameter.bind(&self_rc);
                }

                for parameter in parameters.borrow().iter() {
                    parameter.bind(&self_rc);
                }

                for statement in body.borrow().iter() {
                    statement.bind(&self_rc, locals);
                }
            }
            Expression::Call {
                expression,
                type_arguments,
                arguments,
                parent,
            } => {
                *parent.borrow_mut() = Some(parent_weak);

                expression.borrow().bind(&self_rc);

                for type_argument in type_arguments.borrow().iter() {
                    type_argument.bind(&self_rc);
                }

                for argument in arguments.borrow().iter() {
                    argument.bind(&self_rc);
                }
            }
        }
    }

    pub fn get_name(self: &Rc<Self>) -> String {
        match &**self {
            Expression::Object { .. } => String::from("__object"),
            _ => panic!("Cannot get name of the expression"),
        }
    }

    fn parse_below_call(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
        if try_consume_token(lexer, &TokenType::OpenBrace) {
            let properties = parse_sequence(
                lexer,
                PropertyAssignment::parse,
                TokenType::Comma,
                TokenType::CloseBrace,
            )?;

            Ok(Expression::Object {
                parent: create_empty_parent(),
                properties: create_children(properties),
            })
        } else if try_consume_token(lexer, &TokenType::Function) {
            let name = if Some(&TokenType::Identifier) == lexer.get_type() {
                Some(Identifier::parse(lexer)?)
            } else {
                None
            };

            let type_parameters = if try_consume_token(lexer, &TokenType::LessThan) {
                parse_sequence(
                    lexer,
                    TypeParameter::parse,
                    TokenType::Comma,
                    TokenType::GreaterThan,
                )?
            } else {
                vec![]
            };

            parse_expected(lexer, TokenType::OpenParen)?;

            let parameters = parse_sequence(
                lexer,
                Parameter::parse,
                TokenType::Comma,
                TokenType::CloseParen,
            )?;

            let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

            parse_expected(lexer, TokenType::OpenBrace)?;

            let body = parse_sequence(
                lexer,
                Statement::parse,
                TokenType::Semicolon,
                TokenType::CloseBrace,
            )?;

            Ok(Expression::Function {
                parent: create_empty_parent(),
                name: create_optional_child(name),
                type_parameters: create_children(type_parameters),
                parameters: create_children(parameters),
                typename: create_optional_child(typename),
                body: create_children(body),
                locals: Table::new(),
            })
        } else {
            match lexer.get_type() {
                Some(TokenType::Identifier) => Expression::parse_identifier_or_assignment(lexer),
                Some(TokenType::StringLiteral) | Some(TokenType::NumericLiteral) => {
                    Expression::parse_literal(lexer)
                }
                _ => Err(ParsingError::UnexpectedEndOfFileError),
            }
        }
    }

    fn parse_identifier_or_assignment(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
        let name = Identifier::parse(lexer)?;

        if let Some(expression) = try_parse_prefixed(lexer, Expression::parse, TokenType::Equals) {
            Ok(Expression::Assignment {
                parent: create_empty_parent(),
                name: create_child(name),
                value: create_child(expression),
            })
        } else {
            Ok(Expression::Identifier(create_child(name)))
        }
    }

    fn parse_literal(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
        match lexer.get_type() {
            Some(TokenType::NumericLiteral) => {
                let value = lexer.get().unwrap().text.parse::<i64>().unwrap();
                lexer.next();
                Ok(Expression::NumericLiteral { value })
            }
            Some(TokenType::StringLiteral) => {
                let value = lexer.get().unwrap().text.clone();
                lexer.next();
                Ok(Expression::StringLiteral { value })
            }
            _ => Err(ParsingError::UnexpectedEndOfFileError),
        }
    }
}
