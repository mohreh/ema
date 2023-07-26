use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

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
