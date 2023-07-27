use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::Error, expression::Expression};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub record: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env: Environment = Default::default();
        env.define("nil", Expression::Void);
        env
    }

    pub fn extend(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            parent: Some(parent),
            record: HashMap::new(),
        }
    }

    pub fn from(record: HashMap<String, Expression>) -> Self {
        Environment {
            parent: None,
            record,
        }
    }

    pub fn define(&mut self, name: &str, value: Expression) -> Result<Expression, Error> {
        self.record.insert(name.to_string(), value.clone());
        Ok(value)
    }

    pub fn lookup(&mut self, name: &str) -> Result<Expression, Error> {
        if let Some(value) = self.record.get(name) {
            Ok(value.clone())
        } else {
            self.resolve(name)?
                .borrow()
                .record
                .get(name)
                .ok_or(Error::Reference(format!("{} is not defined", name)))
                .cloned()
        }
    }

    pub fn assign(&mut self, name: &str, new_value: Expression) -> Result<Expression, Error> {
        if let Some(value) = self.record.get_mut(name) {
            *value = new_value.clone();
            Ok(new_value)
        } else {
            self.resolve(name)?
                .borrow_mut()
                .record
                .insert(name.to_string(), new_value.clone())
                .ok_or(Error::Reference(format!("{} is not defined", name)))?;
            Ok(new_value)
        }
    }

    // // implement identifier resolution
    fn resolve(&self, name: &str) -> Result<Rc<RefCell<Self>>, Error> {
        if let Some(parent_env) = &self.parent {
            if parent_env.borrow().record.contains_key(name) {
                return Ok(parent_env.clone());
            }

            parent_env.borrow().resolve(name)
        } else {
            Err(Error::Reference(format!("{} is not defined", name)))
        }
    }
}
