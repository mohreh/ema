use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

// tests for syntactic suger: switch for += ++ -= --

#[test]
fn switch_test() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (var x 10) 
        (switch 
                ((= x 10) (100))
                ((> x 10) 200)
                (else (300))
        )
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(100.0))
    );
}

#[test]
fn shorthand_assign() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (var x 10) 
        (++ x)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(11.0))
    );

    let exp = parse(
        "
    (begin
        (var x 10) 
        (-- x)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(9.0))
    );

    let exp = parse(
        "
    (begin
        (var x 10) 
        (var y 20) 
        (+= x y)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(30.0))
    );

    let exp = parse(
        "
    (begin
        (var x 10)
        (var y -5)
        (-= x y)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(15.0))
    );
}
