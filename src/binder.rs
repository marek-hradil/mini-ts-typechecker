use crate::types::module::Module;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

pub trait Node {}
pub enum Meaning {
    Value,
    Type,
}

pub type Table = HashMap<String, Symbol>;

#[derive(Debug)]
pub struct Symbol {
    delarations: RefCell<Vec<Weak<dyn Node>>>,
}

type Parent = RefCell<Option<Weak<dyn Node>>>;
type Children<T> = RefCell<Vec<Rc<T>>>;
type Child<T> = RefCell<Rc<T>>;
type OptionalChild<T> = RefCell<Option<Rc<T>>>;

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
