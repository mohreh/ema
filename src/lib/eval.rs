use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::environment::Environment;
use crate::error::Error;
use crate::expression::Expression;

pub fn eval_exp(exp: &Expression, env: &mut Rc<RefCell<Environment>>) -> Result<Expression, Error> {
    match exp {
        Expression::Number(num) => Ok(Expression::Number(*num)),

        Expression::String(str) => 
            // if str.bytes().next() == str.bytes().next_back()
            //     && str.bytes().next() == "'".bytes().next() =>
        {
            Ok(Expression::String(str.to_owned()))
        }

        // access variable
        // if var_name_re.is_match(str) 
        Expression::Symbol(str) => env.borrow_mut().lookup(str),

        Expression::List(list) => eval_list(list, env),

        Expression::Boolean(bool) => Ok(Expression::Boolean(*bool)),

        // _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_list(
    list: &Vec<Expression>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    use Expression::*;

    if let Some(head) = list.get(0) {
        match head {
            Symbol(s) => match s.as_str() {
                "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" | "&" | "|" => {
                    eval_binary_op(list, env)
                }
                "var" => eval_define_variable(list, env),
                "set" => eval_assign_variable(list, env),
                "if" => eval_if(list, env),
                "while" => eval_while(list, env),
                _ => eval_exp(head, env),
            },
            // block: sequence of expression
            _ => {
                let mut nested_block_env = Rc::new(RefCell::new(Environment::extend(env.clone())));

                let mut result: Expression = Expression::Boolean(false);
                for exp in list {
                    result = eval_exp(exp, &mut nested_block_env)?;
                }

                Ok(result)
            }
        }
    } else {
        Ok(Expression::Boolean(false))
    }
}

fn eval_while(
    list: &[Expression],
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    let [_tag, condition, body] = &list else {
        return Err(Error::Reason("invalid while statement".to_string()))
    };

    let mut result = Expression::Boolean(false);
    loop {
        if let Expression::Boolean(cond) = eval_exp(condition, env)? {
            if cond {
                result = eval_exp(body, env)?;
            } else {
                break;
            }
        }
    }

    Ok(result)
}

fn eval_if(list: &[Expression], env: &mut Rc<RefCell<Environment>>) -> Result<Expression, Error> {
    let [_tag, condition, consequent, alternate] = &list else {
        return Err(Error::Reason("invalid if statement".to_string()))
    };

    if let Expression::Boolean(cond) = eval_exp(condition, env)? {
        if cond {
            eval_exp(consequent, env)
        } else {
            eval_exp(alternate, env)
        }
    } else {
        Err(Error::Reason("invalid if statement".to_string()))
    }
}

fn eval_define_variable(
    list: &Vec<Expression>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    use Expression::Symbol;

    if list.len() != 3 {
        return Err(Error::Reason("Invalid number of argurments".to_string()));
    }

    if let Symbol(name) = &list[1] {
        let value = eval_exp(&list[2], env)?;
        // let result = env.borrow_mut();
        Ok(env.borrow_mut().define(name, value))
    } else {
        Err(Error::Reason("Invalid defining variable".to_string()))
    }
}

fn eval_assign_variable(
    list: &Vec<Expression>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    use Expression::Symbol;

    if list.len() != 3 {
        return Err(Error::Reason("Invalid number of argurments".to_string()));
    }

    if let Symbol(name) = &list[1] {
        let value = eval_exp(&list[2], env)?;
        env.borrow_mut().assign(name, value)
    } else {
        Err(Error::Reason("Invalid assigning variable".to_string()))
    }
}

fn eval_binary_op(
    list: &[Expression],
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    use Expression::*;

    let head = &list[0];
    let left = eval_exp(&list[1], env)?;
    let right = eval_exp(&list[2], env)?;

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