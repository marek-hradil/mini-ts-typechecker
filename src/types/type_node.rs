use super::{
    parameter::Parameter, property_declaration::PropertyDeclaration, type_parameter::TypeParameter,
    Location,
};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_identifier, parse_sequence, try_consume_token};

#[derive(Debug)]
pub enum TypeNode {
    ObjectLiteralType {
        location: Location,
        properties: Vec<PropertyDeclaration>,
    },
    Identifier {
        text: String,
    },
    SignatureDeclaration {
        location: Location,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<Parameter>,
        typename: Box<TypeNode>,
    },
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
                location: Default::default(),
                properties: properties,
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
                location: Default::default(),
                type_parameters: type_parameters,
                parameters: parameters,
                typename: Box::new(typename),
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
                location: Default::default(),
                type_parameters: Vec::new(),
                parameters: parameters,
                typename: Box::new(typename),
            })
        } else {
            Ok(TypeNode::Identifier {
                text: parse_identifier(lexer)?,
            })
        }
    }
}
