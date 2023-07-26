use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn callback_lambda() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (def on_click (callback) (begin
            (var x 10)
            (var y 20)
            (callback (+ x y))
        ))
        (on_click (lambda data (* data 10)))
        (on_click (lambda (data) (* data 10)))
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(300.0))
    );
}

#[test]
fn immediately_call_lambda() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        ((lambda (x) (* x x)) 4)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(16.0))
    );
}

#[test]
fn save_lambda() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (var square (lambda (x) (* x x))) 
        (square 4)

    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(16.0))
    );
}
