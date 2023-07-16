pub mod environment;
pub mod error;
pub mod expression;

use environment::Environment;
use error::Error;
use expression::Expression;

pub fn eval_exp(exp: &Expression, env: &mut Environment) -> Result<Expression, Error> {
    match exp {
        Expression::Number(num) => Ok(Expression::Number(*num)),
        Expression::String(str)
            if str.bytes().next() == str.bytes().next_back()
                && str.bytes().next() == "'".bytes().next() =>
        {
            println!("{:?}", str.bytes());
            Ok(Expression::String(str[1..str.len() - 1].to_string()))
        }
        Expression::List(list) => eval_list(list, env),

        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_list(list: &Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    use Expression::*;

    let head = &list[0];
    match head {
        String(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return eval_binary_op(&list, env);
            }
            "var" => eval_define_variable(list, env),
            _ => Err(Error::Reason("unimplemented".to_string())),
        },
        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_define_variable(
    list: &Vec<Expression>,
    env: &mut Environment,
) -> Result<Expression, Error> {
    use Expression::String;

    if list.len() != 3 {
        return Err(Error::Reason("Invalid number of argurments".to_string()));
    }

    if let String(name) = &list[1] {
        let value = eval_exp(&list[2], env)?;
        Ok(env.define(name, value))
    } else {
        Err(Error::Reason("Invalid defining variable".to_string()))
    }
}

fn eval_binary_op(list: &Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
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
            _ => todo!(),
        },
        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}
