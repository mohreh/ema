use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::Error, expression::Expression};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub record: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(&self) -> Self {
        Environment {
            parent: Some(Rc::new(RefCell::new(self.clone()))),
            record: HashMap::new(),
        }
    }

    pub fn from(record: HashMap<String, Expression>) -> Self {
        Environment {
            parent: None,
            record,
        }
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
