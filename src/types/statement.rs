use super::Parent;
use super::{expression::Expression, identifier::Identifier, type_node::TypeNode, Node};
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, try_consume_token, try_parse_prefixed};

#[derive(Debug)]
pub enum Statement {
    Var {
        parent: Parent,
        name: Identifier,
        typename: Option<TypeNode>,
        initializer: Expression,
    },
    TypeAlias {
        parent: Parent,
        name: Identifier,
        typename: TypeNode,
    },
    ExpressionStatement {
        parent: Parent,
        expression: Expression,
    },
    Return {
        parent: Parent,
        expression: Expression,
    },
}

impl Node for Statement {}

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
        let name = Identifier::parse(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        parse_expected(lexer, TokenType::Equals)?;

        let initializer = Expression::parse(lexer)?;

        Ok(Statement::Var {
            name,
            typename,
            initializer,
            parent: None,
        })
    }

    fn parse_type_alias(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = Identifier::parse(lexer)?;

        parse_expected(lexer, TokenType::Equals)?;

        let typename = TypeNode::parse(lexer)?;

        Ok(Statement::TypeAlias {
            name,
            typename,
            parent: None,
        })
    }

    fn parse_return(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::Return {
            expression,
            parent: None,
        })
    }

    fn parse_expression_statement(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::ExpressionStatement {
            expression,
            parent: None,
        })
    }
}
