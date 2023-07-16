pub mod error;
pub mod expression;
pub mod literal;

use error::Error;
use expression::Expression;
use literal::Literal;

pub fn eval(exp: Expression) -> Result<Literal, Error> {
    match exp {
        Expression::Literal(l) => match l {
            Literal::Number(num) => Ok(Literal::Number(num)),
            Literal::String(str) => Ok(Literal::String(str[1..str.len() - 1].to_string())),
            _ => Err(Error::Reason("unimplemented".to_string())),
        },
        Expression::List(exp) => match &exp[0] {
            Expression::Literal(l) => match l {
                Literal::String(str) => match str.as_str() {
                    "+" => Err(Error::Reason("unimplemented".to_string())),
                    _ => Err(Error::Reason("unimplemented".to_string())),
                },
                _ => Err(Error::Reason("unimplemented".to_string())),
            },
            _ => Err(Error::Reason("unimplemented".to_string())),
        },
    }
}
