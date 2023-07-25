use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn define_and_call_new_fuction() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (
        (def square (x) (
            (* x x)
        ))
        (square 4)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(16.0))
    );
}

#[test]
fn function_should_capture_outer_variables() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (
        (var x 10)

        (def foo () x) 

        (def bar () (
            (var x 20)
            (+ (foo) x)
        ))

        (bar)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(30.0))
    );
}
