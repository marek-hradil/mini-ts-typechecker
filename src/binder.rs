use crate::{errors::BindingError, types::module::Module};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub trait AstNode {
    fn get_meaning(&self) -> Meaning;
    fn get_name(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub enum Meaning {
    Value,
    Type,
}

pub type Table = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    pub declarations: RefCell<Vec<Weak<dyn AstNode>>>,
}

pub type Parent = RefCell<Option<Weak<dyn AstNode>>>;
pub type Children<T> = RefCell<Vec<Rc<T>>>;
pub type Child<T> = RefCell<Rc<T>>;
pub type OptionalChild<T> = RefCell<Option<Rc<T>>>;

pub fn bind(module: Module) -> Rc<Module> {
    let module_rc = Rc::new(module);

    module_rc.bind();

    module_rc
}

pub fn create_child<T>(node: T) -> Child<T> {
    RefCell::new(Rc::new(node))
}

pub fn create_optional_child<T>(node: Option<T>) -> OptionalChild<T> {
    RefCell::new(node.map(Rc::new))
}

pub fn create_children<T>(nodes: Vec<T>) -> Children<T> {
    RefCell::new(nodes.into_iter().map(Rc::new).collect())
}

pub fn create_empty_parent() -> Parent {
    RefCell::new(None)
}

pub fn declare_symbol(
    locals: &mut Table,
    declaration: &Rc<dyn AstNode>,
) -> Result<(), BindingError> {
    if let Some(symbol) = locals.get(&declaration.get_name()) {
        let mut declarations = symbol.declarations.borrow_mut();
        let other = declarations.iter().find(|d| {
            d.upgrade()
                .map_or_else(|| false, |d| d.get_meaning() == declaration.get_meaning())
        });

        if let Some(_) = other {
            // @todo: Would be nice to correct all of the errors, to show actually useful info
            Err(BindingError::CannotRedeclareError)
        } else {
            declarations.push(Rc::downgrade(declaration));

            Ok(())
        }
    } else {
        locals.insert(
            declaration.get_name(),
            Symbol {
                declarations: RefCell::new(vec![Rc::downgrade(declaration)]),
            },
        );

        Ok(())
    }
}
