use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn define_and_call_new_fuction() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (def square (x) (begin
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
    (begin
        (var x 10)

        (def foo () x) 

        (def bar () (begin
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

#[test]
fn function_with_more_params() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    let exp = parse(
        "
        (begin
            (def sum (x y) (+ x y))
            (sum 2 3)
        )
    ",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(5.0))
    )
}

#[test]
fn inner_closure() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    let exp = parse(
        "
        (begin
            (var val 100)
            (def calc (x y) 
                (begin
                    (var z (+ x y))

                    (def inner (foo)
                        (+ (+ foo z) val) 
                    )

                    inner
                )
            )
            (var fn (calc 20 30))
            (fn 30)
        )
    ",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(180.0))
    )
}
