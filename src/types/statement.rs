use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, parse_identifier, try_consume_token, try_parse_prefixed};
use crate::types::expression::Expression;
use crate::types::type_node::TypeNode;
use crate::types::Location;

#[derive(Debug)]
pub enum Statement {
    Var {
        location: Location,
        name: String,
        typename: Option<TypeNode>,
        initializer: Expression,
    },
    TypeAlias {
        location: Location,
        name: String,
        typename: TypeNode,
    },
    ExpressionStatement {
        location: Location,
        expression: Expression,
    },
    Return {
        location: Location,
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
            location: Default::default(),
        })
    }

    fn parse_type_alias(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = parse_identifier(lexer)?;

        parse_expected(lexer, TokenType::Equals)?;

        let typename = TypeNode::parse(lexer)?;

        Ok(Statement::TypeAlias {
            name,
            typename,
            location: Default::default(),
        })
    }

    fn parse_return(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::Return {
            expression,
            location: Default::default(),
        })
    }

    fn parse_expression_statement(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::ExpressionStatement {
            expression,
            location: Default::default(),
        })
    }
}
