pub mod environment;
pub mod error;
pub mod expression;

use std::{cell::RefCell, rc::Rc};

use environment::Environment;
use error::Error;
use expression::Expression;
use regex::Regex;

pub fn eval_exp(exp: &Expression, env: &mut Rc<RefCell<Environment>>) -> Result<Expression, Error> {
    let var_name_re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*?$").unwrap();

    match exp {
        Expression::Number(num) => Ok(Expression::Number(*num)),

        // if expression is just a string like "hello world!"
        Expression::String(str)
            if str.bytes().next() == str.bytes().next_back()
                && str.bytes().next() == "'".bytes().next() =>
        {
            Ok(Expression::String(str[1..str.len() - 1].to_string()))
        }

        // access variable
        Expression::String(str) if var_name_re.is_match(str) => env.borrow_mut().lookup(str),

        Expression::List(list) => eval_list(list, env),

        Expression::Boolean(bool) => Ok(Expression::Boolean(*bool)),

        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_list(
    list: &Vec<Expression>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    use Expression::*;

    if let Some(head) = list.get(0) {
        match head {
            String(s) => match s.as_str() {
                "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_op(list, env),
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
    use Expression::String;

    if list.len() != 3 {
        return Err(Error::Reason("Invalid number of argurments".to_string()));
    }

    if let String(name) = &list[1] {
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
    use Expression::String;

    if list.len() != 3 {
        return Err(Error::Reason("Invalid number of argurments".to_string()));
    }

    if let String(name) = &list[1] {
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

    let left_val = match left {
        Number(num) => num,
        _ => {
            return Err(Error::Reason(
                "+ op unimplemented for given value types".to_string(),
            ))
        }
    };

    let right_val = match right {
        Number(num) => num,
        _ => {
            return Err(Error::Reason(
                "+ op unimplemented for given value types".to_string(),
            ))
        }
    };

    match head {
        Expression::String(str) => match str.as_str() {
            "+" => Ok(Number(left_val + right_val)),
            "-" => Ok(Number(left_val - right_val)),
            "*" => Ok(Number(left_val * right_val)),
            "/" => Ok(Number(left_val / right_val)),
            // todo
            "%" => Ok(Number(left_val % right_val)),
            ">" => Ok(Boolean(left_val > right_val)),
            ">=" => Ok(Boolean(left_val >= right_val)),
            "<" => Ok(Boolean(left_val < right_val)),
            "<=" => Ok(Boolean(left_val <= right_val)),
            "==" => Ok(Boolean(left_val == right_val)),
            "!=" => Ok(Boolean(left_val != right_val)),
            _ => todo!(),
        },
        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}
