use std::fmt::Debug;

pub mod expression;
pub mod module;
pub mod parameter;
pub mod property_assignment;
pub mod property_declaration;
pub mod statement;
pub mod type_node;
pub mod type_parameter;

pub trait Node {}

trait WithLocation {
    fn set_parent(&mut self, parent: Box<dyn Node>) -> ();
}

impl Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node")
    }
}

#[derive(Debug)]
pub struct Location {
    parent: Option<Box<dyn Node>>,
}

impl Default for Location {
    fn default() -> Self {
        Location { parent: None }
    }
}
