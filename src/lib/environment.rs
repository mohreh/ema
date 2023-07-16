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

    pub fn lookup(&self, name: &str) -> Result<Expression, Error> {
        match self.record.get(name) {
            Some(var) => Ok(var.clone()),
            None => Err(Error::Reason(format!("variable {} is not defined", name))),
        }
    }

    pub fn define(&mut self, name: &str, value: Expression) -> Expression {
        self.record.insert(name.to_string(), value.clone());
        value
    }
}
