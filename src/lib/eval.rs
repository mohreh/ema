use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::{environment::Environment, error::Error, transform::transform_switch_to_if};
use crate::{expression::Expression, transform::transform_def_to_var_lambda};

#[derive(Default, Debug)]
pub struct Evaluator {
    env_arena: Vec<Rc<RefCell<Environment>>>,
}

impl Evaluator {
    pub fn eval_exp(
        &mut self,
        exp: &Expression,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        match exp {
            Expression::Number(num) => Ok(Expression::Number(*num)),

            Expression::String(str) => Ok(Expression::String(str.to_owned())),

            // access variable
            Expression::Symbol(str) => env.borrow_mut().lookup(str),

            Expression::List(list) => self.eval_list(list, env),

            Expression::Boolean(bool) => Ok(Expression::Boolean(*bool)),

            Expression::Function(params, body, env_idx) => Ok(Expression::Function(
                params.to_vec(),
                body.clone(),
                *env_idx,
            )),
            // _ => Err(Error::Reason("unimplemented".to_string())),
        }
    }

    fn eval_list(
        &mut self,
        list: &Vec<Expression>,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::*;

        if let Some(head) = list.get(0) {
            match head {
                Symbol(s) => match s.as_str() {
                    "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" | "&" | "|" => {
                        self.eval_binary_op(list, env)
                    }
                    "var" => self.eval_define_variable(list, env),
                    "set" => self.eval_assign_variable(list, env),
                    "if" => self.eval_if(list, env),
                    "switch" => self.eval_exp(&transform_switch_to_if(list)?, env),
                    "while" => self.eval_while(list, env),
                    "def" => self.eval_define_function(list, env),
                    "begin" => self.eval_block(list, env),
                    "lambda" => self.eval_define_lambda(list, env),
                    "print" => self.eval_print(list, env),
                    // user defined functions or variables
                    _ => {
                        if let Ok(Function(params, body, env_idx)) = self.eval_exp(head, env) {
                            // static scope
                            let mut activation_env = Rc::new(RefCell::new(Environment::extend(
                                self.env_arena
                                    .get(env_idx)
                                    .ok_or(Error::Reason("unexpected error".to_string()))?
                                    .clone(),
                            )));

                            return self.eval_function_body(
                                list,
                                params,
                                body,
                                env,
                                &mut activation_env,
                            );
                        }

                        self.eval_exp(head, env)
                    }
                },
                // immediately call function
                _ => {
                    let head_evaluated = self.eval_exp(head, env)?;
                    if let Expression::Function(params, body, env_idx) = head_evaluated {
                        self.eval_function_body(
                            list,
                            params,
                            body,
                            env,
                            &mut Rc::new(RefCell::new(Environment::extend(
                                self.env_arena
                                    .get(env_idx)
                                    .ok_or(Error::Reason("unexpected error".to_string()))?
                                    .clone(),
                            ))),
                        )
                    } else {
                        Ok(head_evaluated)
                    }
                }
            }
        } else {
            Ok(Expression::Boolean(false))
        }
    }

    // block: sequence of expression
    fn eval_block(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let mut nested_block_env = Rc::new(RefCell::new(Environment::extend(env.clone())));

        let mut result: Expression = Expression::Boolean(false);
        if let Some((_tag, rest)) = list.split_first() {
            for exp in rest {
                result = self.eval_exp(exp, &mut nested_block_env)?;
            }
        }

        Ok(result)
    }

    fn eval_while(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, condition, body] = &list else {
        return Err(Error::Invalid("invalid while statement".to_string()))
    };

        let mut result = Expression::Boolean(false);
        loop {
            if let Expression::Boolean(cond) = self.eval_exp(condition, env)? {
                if cond {
                    result = self.eval_exp(body, env)?;
                } else {
                    break;
                }
            } else {
                return Err(Error::Invalid("invalid while statement".to_string()));
            }
        }

        Ok(result)
    }

    fn eval_if(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, condition, consequent, alternate] = &list else {
        return Err(Error::Invalid("invalid if statement".to_string()))
    };

        if let Expression::Boolean(cond) = self.eval_exp(condition, env)? {
            if cond {
                self.eval_exp(consequent, env)
            } else {
                self.eval_exp(alternate, env)
            }
        } else {
            Err(Error::Invalid("invalid if statement".to_string()))
        }
    }

    fn eval_define_function(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        // JIT-transpile to a variable declaration
        self.eval_exp(&transform_def_to_var_lambda(list)?, env)
    }

    fn eval_define_lambda(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, params, body] = &list else {
        return Err(Error::Invalid("invalid defining lambda.".to_string()))
    };

        let params = {
            match params {
                Expression::List(list) => list
                    .iter()
                    .map(|param| match param {
                        Expression::Symbol(name) => Ok(name.clone()),
                        _ => Err(Error::Invalid("invalid params for lambda".to_string())),
                    })
                    .collect::<Result<Vec<String>, Error>>()?,
                Expression::Symbol(name) => vec![name.clone()],
                _ => return Err(Error::Invalid("invalid params for lambda".to_string())),
            }
        };

        self.env_arena.push(env.clone());

        Ok(Expression::Function(
            params,
            Rc::new(RefCell::new(body.clone())),
            self.env_arena.len() - 1,
        ))
    }

    fn eval_function_body(
        &mut self,
        list: &[Expression],
        params: Vec<String>,
        body: Rc<RefCell<Expression>>,
        env: &mut Rc<RefCell<Environment>>,
        activation_env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        if let Some((func_name, args)) = list.split_first() {
            if params.len() != args.len() {
                return Err(Error::Reason(format!(
                    "function {} took {} argurments you provide {}",
                    func_name,
                    params.len(),
                    args.len()
                )));
            }

            for (idx, param_name) in params.iter().enumerate() {
                let _ = activation_env.borrow_mut().define(
                    param_name,
                    self.eval_exp(
                        args.get(idx)
                            .ok_or(Error::Reason("unexpected error".to_string()))?,
                        env,
                    )?,
                )?;
            }

            self.eval_exp(&body.borrow(), activation_env)
        } else {
            Err(Error::Reason("unexpected error".to_string()))
        }
    }

    fn eval_define_variable(
        &mut self,
        list: &Vec<Expression>,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::Symbol;

        if list.len() != 3 {
            return Err(Error::Invalid("Invalid number of argurments".to_string()));
        }

        if let Symbol(name) = &list[1] {
            let value = self.eval_exp(&list[2], env)?;
            env.borrow_mut().define(name, value)
        } else {
            Err(Error::Invalid("Invalid defining variable".to_string()))
        }
    }

    fn eval_assign_variable(
        &mut self,
        list: &Vec<Expression>,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::Symbol;

        if list.len() != 3 {
            return Err(Error::Invalid("Invalid number of argurments".to_string()));
        }

        if let Symbol(name) = &list[1] {
            let value = self.eval_exp(&list[2], env)?;
            env.borrow_mut().assign(name, value)
        } else {
            Err(Error::Invalid("Invalid assigning variable".to_string()))
        }
    }

    fn eval_print(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        if let Some((_, args)) = list.split_first() {
            for arg in args {
                let exp = self.eval_exp(arg, env)?;
                print!("{}", exp)
            }
            println!();

            Ok(Expression::Boolean(true))
        } else {
            Err(Error::Reason("unexpected error.".to_string()))
        }
    }

    fn eval_binary_op(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::*;

        let head = &list[0];
        let left = self.eval_exp(&list[1], env)?;
        let right = self.eval_exp(&list[2], env)?;

        match head {
            Expression::Symbol(str) => match str.as_str() {
                "+" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val + right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } + right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val + if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } + if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => Ok(String(left_val + &right_val)),
                    _ => Err(Error::Invalid("invalid type for + operator".to_string())),
                },
                "-" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val - right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } - right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val - if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } - if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for - operator".to_string())),
                },
                "*" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val * right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } * right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val * if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } * if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for * operator".to_string())),
                },

                "/" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val / right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } / right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val / if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } / if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for / operator".to_string())),
                },
                "%" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val % right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } % right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val % if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } % if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for % operator".to_string())),
                },
                ">" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val > right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } > right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val > if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } > if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Greater))
                    }
                    _ => Err(Error::Invalid("invalid type for > operator".to_string())),
                },

                ">=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val >= right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } >= right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val >= if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } >= if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for >= operator".to_string())),
                },
                "<" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val < right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } < right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val < if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } < if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Less))
                    }
                    _ => Err(Error::Invalid("invalid type for < operator".to_string())),
                },

                "<=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val <= right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } <= right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val <= if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } <= if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for <= operator".to_string())),
                },
                "=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val == right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } == right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val == if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } == if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Equal))
                    }
                    _ => Err(Error::Invalid("invalid type for == operator".to_string())),
                },

                "!=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val != right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } != right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val != if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } != if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) != Ordering::Equal))
                    }
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },

                "&" => match (left, right) {
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(left_val & right_val)),
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },

                "|" => match (left, right) {
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(left_val | right_val)),
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },
                _ => todo!(),
            },
            _ => Err(Error::Invalid("invalid operator".to_string())),
        }
    }
}
