use super::Parent;
use super::{
    identifier::Identifier, parameter::Parameter, property_declaration::PropertyDeclaration,
    type_parameter::TypeParameter, Node,
};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_sequence, try_consume_token};

#[derive(Debug)]
pub enum TypeNode {
    ObjectLiteralType {
        parent: Parent,
        properties: Vec<PropertyDeclaration>,
    },
    Identifier(Identifier),
    SignatureDeclaration {
        parent: Parent,
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<Parameter>,
        typename: Box<TypeNode>,
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
                parent: None,
                properties,
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
                parent: None,
                type_parameters,
                parameters,
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
                parent: None,
                type_parameters: Vec::new(),
                parameters,
                typename: Box::new(typename),
            })
        } else {
            Ok(TypeNode::Identifier(Identifier::parse(lexer)?))
        }
    }
}
