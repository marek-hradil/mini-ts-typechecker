use std::{cell::RefCell, fmt::Debug, rc::Weak};

pub mod expression;
pub mod identifier;
pub mod module;
pub mod parameter;
pub mod property_assignment;
pub mod property_declaration;
pub mod statement;
pub mod type_node;
pub mod type_parameter;

pub trait Node: Debug {}

type Parent = Option<RefCell<Weak<dyn Node>>>;
