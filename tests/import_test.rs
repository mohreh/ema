use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn import() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "

    (begin 
        (import Math)
        (var x ((prop Math abs) -10))
        (var abs (prop Math abs))
        (var square (prop Math square))
        (var y (square (abs -20)))
        (- (prop Math MAX_VAL) (+ x y))
    )
",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(590.0))
    );
}

#[test]
fn import_single_prop() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "

    (begin 
        (import abs Math)
        (+ (abs -10) 20)
    )
",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(30.0))
    );
}

#[test]
fn import_more_prop() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "

    (begin 
        (import (square abs) Math)
        (square (- (abs -10) (square 2)))
    )
",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(36.0))
    );
}
