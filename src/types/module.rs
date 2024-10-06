use crate::binder::{create_children, AstNode, Children, Meaning, Table};
use crate::errors::{BindingError, ParsingError};
use crate::lexer::{Lexer, TokenType};
use crate::parser::parse_sequence;
use crate::types::statement::Statement;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Module {
    pub statements: Children<Statement>,
    pub locals: RefCell<Table>,
}

impl AstNode for Module {
    fn get_meaning(&self) -> Meaning {
        Meaning::Value
    }

    fn get_name(&self) -> String {
        String::from("__module")
    }
}

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
            locals: RefCell::new(Table::new()),
        };

        Ok(module)
    }

    pub fn bind(self: &Rc<Self>) -> Result<(), BindingError> {
        let self_rc: Rc<dyn AstNode> = self.clone();
        for statement_rc in self.statements.borrow().iter() {
            statement_rc.bind(&self_rc, &mut self.locals.borrow_mut())?;
        }

        Ok(())
    }
}
