use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::Error, expression::Expression};

#[derive(Default, Debug, PartialEq)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub record: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(
        parent: Option<Rc<RefCell<Environment>>>,
        record: HashMap<String, Expression>,
    ) -> Self {
        Environment { parent, record }
    }

    pub fn lookup(&self, name: &str) -> Result<Expression, Error> {
        match self.record.get(name) {
            Some(var) => Ok(var.clone()),
            None => Err(Error::Reference(format!(
                "variable {} is not defined",
                name
            ))),
        }
    }

    pub fn define(&mut self, name: &str, value: Expression) -> Expression {
        self.record.insert(name.to_string(), value.clone());
        value
    }
}
