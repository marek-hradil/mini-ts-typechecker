use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub mod expression;
pub mod identifier;
pub mod module;
pub mod parameter;
pub mod property_assignment;
pub mod property_declaration;
pub mod statement;
pub mod type_node;
pub mod type_parameter;

pub trait Node {}

type Parent = RefCell<Option<Weak<dyn Node>>>;
type Children<T> = RefCell<Vec<Rc<T>>>;
type Child<T> = RefCell<Rc<T>>;
type OptionalChild<T> = RefCell<Option<Rc<T>>>;

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
