use std::rc::Rc;

use super::{create_children, Children};
use super::{statement::Statement, Node};
use crate::binder::Table;
use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_sequence;

#[derive(Debug)]
pub struct Module {
    pub statements: Children<Statement>,
    pub locals: Table,
}

impl Node for Module {}

impl Module {
    pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
        let statements = parse_sequence(
            lexer,
            Statement::parse,
            TokenType::Semicolon,
            TokenType::EOF,
        )?;

        let module = Module {
            statements: create_children(statements),
            locals: Table::new(),
        };

        Ok(module)
    }

    pub fn bind(self: &Rc<Self>) {
        let self_rc: Rc<dyn Node> = self.clone();
        for statement_rc in self.statements.borrow().iter() {
            statement_rc.bind(&self_rc, &self.locals);
        }
    }
}
