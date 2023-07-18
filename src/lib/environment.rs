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
        self.resolve(name)?
            .record
            .get(name)
            .ok_or(Error::Reference(format!(
                "variable {} is not defined",
                name
            )))
            .cloned()
    }

    // fix bug later
    // implement identifier resolution
    fn resolve(&self, name: &str) -> Result<Self, Error> {
        if self.record.contains_key(name) {
            return Ok(self.clone());
        }

        if let Some(parent_env) = &self.parent {
            parent_env.take().resolve(name)
        } else {
            Err(Error::Reference(format!(
                "variable {} is not defined",
                name
            )))
        }
    }

    pub fn define(&mut self, name: &str, value: Expression) -> Expression {
        self.record.insert(name.to_string(), value.clone());
        value
    }
}
