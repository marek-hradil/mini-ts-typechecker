use crate::binder::{
    create_child, create_empty_parent, create_optional_child, declare_symbol, Child, OptionalChild,
    Parent,
};
use crate::binder::{AstNode, Meaning, Table};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::{Lexer, TokenType};
use crate::parser::{parse_expected, try_consume_token, try_parse_prefixed};
use crate::types::{expression::Expression, identifier::Identifier, type_node::TypeNode};
use std::rc::Rc;

#[derive(Debug)]
pub enum Statement {
    Var {
        parent: Parent,
        name: Child<Identifier>,
        typename: OptionalChild<TypeNode>,
        initializer: Child<Expression>,
    },
    TypeAlias {
        parent: Parent,
        name: Child<Identifier>,
        typename: Child<TypeNode>,
    },
    ExpressionStatement {
        parent: Parent,
        expression: Child<Expression>,
    },
    Return {
        parent: Parent,
        expression: Child<Expression>,
    },
}

impl AstNode for Statement {
    fn get_meaning(&self) -> Meaning {
        match self {
            Statement::Var { .. } => Meaning::Value,
            Statement::TypeAlias { .. } => Meaning::Type,
            Statement::ExpressionStatement { .. } => Meaning::Value,
            Statement::Return { .. } => Meaning::Value,
        }
    }

    fn get_name(&self) -> String {
        match self {
            Statement::TypeAlias { name, .. } => name.borrow().text.clone(),
            Statement::Var { name, .. } => name.borrow().text.clone(),
            Statement::ExpressionStatement { .. } => {
                panic!("Cannot get name of expression statement")
            }
            Statement::Return { .. } => {
                panic!("Cannot get name of return statement")
            }
        }
    }
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

    pub fn bind(
        self: &Rc<Self>,
        parent: &Rc<dyn AstNode>,
        locals: &mut Table,
    ) -> Result<(), BindingError> {
        let self_rc = Rc::clone(self) as Rc<dyn AstNode>;
        let parent_weak = Rc::downgrade(parent);

        match &**self {
            Statement::Var {
                parent,
                name,
                typename,
                initializer,
                ..
            } => {
                *parent.borrow_mut() = Some(parent_weak);
                name.borrow().bind(&self_rc)?;
                initializer.borrow().bind(&self_rc)?;

                if let Some(type_node_rc) = typename.borrow().as_ref() {
                    type_node_rc.bind(&self_rc)?;
                }

                declare_symbol(locals, &self_rc)
            }
            Statement::TypeAlias {
                parent,
                name,
                typename,
            } => {
                *parent.borrow_mut() = Some(parent_weak);
                name.borrow().bind(&self_rc)?;
                typename.borrow().bind(&self_rc)?;

                declare_symbol(locals, &self_rc)
            }
            Statement::ExpressionStatement { parent, expression } => {
                *parent.borrow_mut() = Some(parent_weak);
                expression.borrow().bind(&self_rc)?;

                Ok(())
            }
            Statement::Return { parent, expression } => {
                *parent.borrow_mut() = Some(parent_weak);
                expression.borrow().bind(&self_rc)?;

                Ok(())
            }
        }
    }

    fn parse_var(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = Identifier::parse(lexer)?;
        let typename = try_parse_prefixed(lexer, TypeNode::parse, TokenType::Colon);

        parse_expected(lexer, TokenType::Equals)?;

        let initializer = Expression::parse(lexer)?;

        Ok(Statement::Var {
            name: create_child(name),
            typename: create_optional_child(typename),
            initializer: create_child(initializer),
            parent: create_empty_parent(),
        })
    }

    fn parse_type_alias(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let name = Identifier::parse(lexer)?;

        parse_expected(lexer, TokenType::Equals)?;

        let typename = TypeNode::parse(lexer)?;

        Ok(Statement::TypeAlias {
            name: create_child(name),
            typename: create_child(typename),
            parent: create_empty_parent(),
        })
    }

    fn parse_return(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::Return {
            expression: create_child(expression),
            parent: create_empty_parent(),
        })
    }

    fn parse_expression_statement(lexer: &mut Lexer) -> Result<Statement, ParsingError> {
        let expression = Expression::parse(lexer)?;

        Ok(Statement::ExpressionStatement {
            expression: create_child(expression),
            parent: create_empty_parent(),
        })
    }
}
