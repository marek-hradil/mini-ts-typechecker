use super::{
    identifier::Identifier, parameter::Parameter, property_assignment::PropertyAssignment,
    statement::Statement, type_node::TypeNode, type_parameter::TypeParameter, Location, Node,
};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{
    parse_expected, parse_identifier, parse_sequence, try_consume_token, try_parse_prefixed,
};

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    NumericLiteral {
        value: i64,
    },
    StringLiteral {
        value: String,
    },
    Assignment {
        location: Location,
        name: Identifier,
        value: Box<Expression>,
    },
    Object {
        location: Location,
        properties: Vec<PropertyAssignment>,
    },
    Function {
        location: Location,
        name: Option<Identifier>,
        type_parameters: Option<Vec<TypeParameter>>,
        parameters: Vec<Parameter>,
        typename: Option<TypeNode>,
        body: Vec<Statement>,
    },
    Call {
        location: Location,
        expression: Box<Expression>,
        type_arguments: Option<Vec<TypeNode>>,
        arguments: Vec<Box<Expression>>,
    },
}

impl Node for Expression {}
impl Expression {
    pub fn parse(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
        let expression = Expression::parse_below_call(lexer)?;

        let type_arguments = if try_consume_token(lexer, &TokenType::LessThan) {
            Some(parse_sequence(
                lexer,
                TypeNode::parse,
                TokenType::Comma,
                TokenType::GreaterThan,
            )?)
        } else {
            None
        };

        if try_consume_token(lexer, &TokenType::OpenParen) {
            let arguments = parse_sequence(
                lexer,
                Expression::parse,
                TokenType::Comma,
                TokenType::CloseParen,
            )?;

            Ok(Expression::Call {
                location: Default::default(),
                expression: Box::new(expression),
                type_arguments: type_arguments,
                arguments: arguments.into_iter().map(Box::new).collect(),
            })
        } else {
            Ok(expression)
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
                location: Default::default(),
                properties: properties,
            })
        } else if try_consume_token(lexer, &TokenType::Function) {
            let name = if Some(&TokenType::Identifier) == lexer.get_type() {
                Some(Identifier {
                    text: parse_identifier(lexer)?,
                    location: Default::default(),
                })
            } else {
                None
            };

            let type_parameters = if try_consume_token(lexer, &TokenType::LessThan) {
                Some(parse_sequence(
                    lexer,
                    TypeParameter::parse,
                    TokenType::Comma,
                    TokenType::GreaterThan,
                )?)
            } else {
                None
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
                location: Default::default(),
                name: name,
                type_parameters: type_parameters,
                parameters: parameters,
                typename: typename,
                body: body,
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
        let text = parse_identifier(lexer)?;

        if let Some(expression) = try_parse_prefixed(lexer, Expression::parse, TokenType::Equals) {
            Ok(Expression::Assignment {
                location: Default::default(),
                name: Identifier {
                    text: text,
                    location: Default::default(),
                },
                value: Box::new(expression),
            })
        } else {
            Ok(Expression::Identifier(Identifier {
                text: text,
                location: Default::default(),
            }))
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
