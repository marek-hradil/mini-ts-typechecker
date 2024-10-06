use std::rc::{Rc, Weak};

use crate::types::{module::Module, statement::Statement, Node};

pub enum Meaning {
    Value,
    Type,
}

pub fn bind(module: Module) -> Rc<Module> {
    let module_rc = Rc::new(module);

    module_rc.bind();

    module_rc
}
