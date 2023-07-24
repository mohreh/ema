use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::eval_exp, expression::Expression, parser::parse};

#[test]
fn define_and_call_new_fuction() {
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (
        (def square (x) (
            (* x x)
        ))
    )",
    );

    assert_eq!(
        eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Symbol("square".to_string()))
    );
}
