use super::{Location, Node};

#[derive(Debug)]
pub struct Identifier {
    pub location: Location,
    pub text: String,
}

impl Node for Identifier {}

impl Identifier {}
