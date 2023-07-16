use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expression::Expression;

#[derive(Default, Debug)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    record: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define(&mut self, name: &str, value: Expression) {
        self.record.insert(name.to_string(), value);
    }
}
