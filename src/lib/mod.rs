pub mod error;
pub mod expression;
pub mod literal;

use error::Error;
use expression::Expression;
use literal::Literal;

pub struct Ema;

impl Ema {
    pub fn eval(exp: Expression) -> Result<Literal, Error> {
        match exp {
            Expression::Literal(l) => match l {
                Literal::Number(num) => Ok(Literal::Number(num)),
                Literal::String(str) => Ok(Literal::String(str)),
                _ => Err(Error::Reason("unimplemented".to_string())),
            },
            Expression::List(exp) => {
                //

                return Err(Error::Reason("unimplemented".to_string()));
            }
        }
    }
}
