pub mod error;
pub mod expression;

use error::Error;
use expression::Expression;

pub fn eval_exp(exp: &Expression) -> Result<Expression, Error> {
    match exp {
        Expression::Number(num) => Ok(Expression::Number(*num)),
        Expression::String(str) => Ok(Expression::String(str[1..str.len() - 1].to_string())),
        Expression::List(list) => eval_list(list.to_vec()),

        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}

fn eval_list(list: Vec<Expression>) -> Result<Expression, Error> {
    use Expression::*;

    let head = &list[0];
    let left = eval_exp(&list[1])?;
    let right = eval_exp(&list[2])?;

    match head {
        Expression::String(str) => match str.as_str() {
            "+" => match (left, right) {
                (Number(l), Number(r)) => Ok(Number(r + l)),
                (String(l), String(r)) => Ok(String(l + &r)),
                _ => Err(Error::Reason(
                    "+ op unimplemented for given value types".to_string(),
                )),
            },
            _ => todo!(),
        },
        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}
