pub mod error;
pub mod expression;

use error::Error;
use expression::Expression;

pub fn eval_exp(exp: Expression) -> Result<Expression, Error> {
    match exp {
        Expression::Number(num) => Ok(Expression::Number(num)),
        Expression::String(str) => Ok(Expression::String(str[1..str.len() - 1].to_string())),
        Expression::List(exp) => {
            let head = &exp[0];
            match head {
                Expression::String(str) => match str.as_str() {
                    "+" => {
                        todo!()
                    }
                    _ => todo!(),
                },
                _ => Err(Error::Reason("unimplemented".to_string())),
            }
        }

        _ => Err(Error::Reason("unimplemented".to_string())),
    }
}
