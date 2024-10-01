use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};

pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
    lexer.next();
    Module::parse(lexer)
}

fn parse_identifier(lexer: &mut Lexer) -> Result<String, ParsingError> {
    match lexer.get() {
        Some(token) if token.token_type == TokenType::Identifier => {
            lexer.next();
            Ok(token.text.clone())
        }
        _ => Err(ParsingError::UnexpectedEndOfFileError),
    }
}

fn parse_sequence<T>(
    lexer: &mut Lexer,
    parse_element: fn(&mut Lexer) -> Result<T, ParsingError>,
    separator: TokenType,
    terminator: TokenType,
) -> Result<Vec<T>, ParsingError> {
    let mut seq = Vec::new();
    while !try_consume_token(lexer, &terminator) {
        match parse_element(lexer) {
            Ok(element) => seq.push(element),
            Err(err) => return Err(err),
        };

        try_consume_token(lexer, &separator);
    }

    Ok(seq)
}

fn try_consume_token(lexer: &mut Lexer, expected: &TokenType) -> bool {
    let ok = match lexer.get_type() {
        Some(token_type) => token_type == expected,
        _ => false,
    };

    if ok {
        lexer.next();
    }

    ok
}

fn try_parse_prefixed<T>(
    lexer: &mut Lexer,
    parse_element: fn(&mut Lexer) -> Result<T, ParsingError>,
    prefix: TokenType,
) -> Option<T> {
    if try_consume_token(lexer, &prefix) {
        match parse_element(lexer) {
            Ok(element) => Some(element),
            Err(_) => None,
        }
    } else {
        None
    }
}

fn parse_expected(lexer: &mut Lexer, expected_type: TokenType) -> Result<(), ParsingError> {
    match lexer.get_type() {
        Some(token_type) if token_type == &expected_type => Ok({
            lexer.next();
        }),
        _ => Err(ParsingError::UnexpectedEndOfFileError),
    }
}

#[derive(Debug)]
pub struct Module {
    statements: Vec<Statement>,
}

impl Module {
    pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
        let statements = parse_sequence(
            lexer,
            Statement::parse,
            TokenType::Semicolon,
            TokenType::EOF,
        )?;

        Ok(Module { statements })
    }
}

#[derive(Debug)]
enum Statement {
    Var {
        name: String,
        typename: Option<TypeNode>,
        initializer: Expression,
    },
    TypeAlias {
        name: String,
        typename: TypeNode,
    },
    ExpressionStatement {
        expression: Expression,
    },
    Return {
        expression: Expression,
    },
}

impl Statement {
    pub fn parse(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        if try_consume_token(lexer, &TokenType::Var) {
            Statement::parse_var(lexer)
        } else if try_consume_token(lexer, &TokenType::Type) {
            Statement::parse_type_alias(lexer)
        } else if try_consume_token(lexer, &TokenType::Return) {
            Statement::parse_return(lexer)
        } else {
            Statement::parse_expression_statement(lexer)
        }
    }

    fn parse_var(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        parse_expected(lexer, TokenType::Equals)?;

        let initializer = Expression::parse(lexer)?;

        Ok(Statement::Var {
            name,
            typename,
            initializer,
        })
    }

    fn parse_type_alias(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = parse_identifier(lexer)?;

        parse_expected(lexer, TokenType::Equals)?;

        let typename = TypeNode::parse(lexer)?;

        Ok(Statement::TypeAlias { name, typename })
    }

    fn parse_return(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::Return { expression })
    }

    fn parse_expression_statement(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::ExpressionStatement { expression })
    }
}

#[derive(Debug)]
enum TypeNode {
    ObjectLiteralType {
        properties: Vec<PropertyDeclaration>,
    },
    Identifier {
        text: String,
    },
    SignatureDeclaration {
        type_parameters: Vec<TypeParameter>,
        parameters: Vec<Parameter>,
        typename: Box<TypeNode>,
    },
}

impl TypeNode {
    fn parse(lexer: &mut Lexer) -> Result<TypeNode, ParsingError> {
        if try_consume_token(lexer, &TokenType::OpenBrace) {
            let properties = parse_sequence(
                lexer,
                PropertyDeclaration::parse,
                TokenType::Comma,
                TokenType::CloseBrace,
            )?;

            Ok(TypeNode::ObjectLiteralType {
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

#[derive(Debug)]
enum Expression {
    Identifier {
        text: String,
    },
    NumericLiteral {
        value: i64,
    },
    StringLiteral {
        value: String,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    Object {
        properties: Vec<PropertyAssignment>,
    },
    Function {
        name: Option<String>,
        type_parameters: Option<Vec<TypeParameter>>,
        parameters: Vec<Parameter>,
        typename: Option<TypeNode>,
        body: Vec<Statement>,
    },
    Call {
        expression: Box<Expression>,
        type_arguments: Option<Vec<TypeNode>>,
        arguments: Vec<Box<Expression>>,
    },
}

impl Expression {
    fn parse(lexer: &mut Lexer) -> Result<Expression, ParsingError> {
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
                properties: properties,
            })
        } else if try_consume_token(lexer, &TokenType::Function) {
            let name = if Some(&TokenType::Identifier) == lexer.get_type() {
                Some(parse_identifier(lexer)?)
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
        let name = parse_identifier(lexer)?;

        if let Some(expression) = try_parse_prefixed(lexer, Expression::parse, TokenType::Equals) {
            Ok(Expression::Assignment {
                name,
                value: Box::new(expression),
            })
        } else {
            Ok(Expression::Identifier { text: name })
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

#[derive(Debug)]
struct Parameter {
    name: String,
    typename: Option<TypeNode>,
}

impl Parameter {
    fn parse(lexer: &mut Lexer) -> Result<Parameter, ParsingError> {
        let name = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(Parameter {
            name: name,
            typename: typename,
        })
    }
}

#[derive(Debug)]
struct TypeParameter {
    name: String,
}

impl TypeParameter {
    fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = parse_identifier(lexer)?;

        Ok(TypeParameter { name })
    }
}

#[derive(Debug)]
struct PropertyDeclaration {
    name: String,
    typename: Option<TypeNode>,
}

impl PropertyDeclaration {
    fn parse(lexer: &mut Lexer) -> Result<PropertyDeclaration, ParsingError> {
        let name = parse_identifier(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        Ok(PropertyDeclaration { name, typename })
    }
}

#[derive(Debug)]
struct PropertyAssignment {
    name: String,
    value: Expression,
}

impl PropertyAssignment {
    fn parse(lexer: &mut Lexer) -> Result<PropertyAssignment, ParsingError> {
        let name = parse_identifier(lexer)?;
        parse_expected(lexer, TokenType::Colon)?;

        let value = Expression::parse(lexer)?;

        Ok(PropertyAssignment { name, value })
    }
}
