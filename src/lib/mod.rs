pub mod error;
pub mod expression;

use error::Error;
use expression::Expression;

pub fn eval_exp(exp: &Expression) -> Result<Expression, Error> {
    match exp {
        Expression::Number(num) => Ok(Expression::Number(*num)),
        Expression::String(str)
            if str.bytes().next() == str.bytes().next_back()
                && str.bytes().next() == "'".bytes().next() =>
        {
            println!("{:?}", str.bytes());
            Ok(Expression::String(str[1..str.len() - 1].to_string()))
        }
        Expression::List(list) => eval_list(list.to_vec()),

        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_list(list: Vec<Expression>) -> Result<Expression, Error> {
    use Expression::*;

    let head = &list[0];
    let left = eval_exp(&list[1])?;
    let right = eval_exp(&list[2])?;

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
