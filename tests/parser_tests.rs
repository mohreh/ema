use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::eval_exp, expression::Expression, parser::parser};

#[test]
fn parser_test() {
    let res = parser("(var x (+ (* 2 2) (+ 2 5)))");
    assert_eq!(
        res,
        Ok(Expression::List(vec![Expression::List(vec![
            Expression::Symbol("var".to_string()),
            Expression::Symbol("x".to_string()),
            Expression::List(vec![
                Expression::Symbol("+".to_string()),
                Expression::List(vec![
                    Expression::Symbol("*".to_string()),
                    Expression::Number(2.0),
                    Expression::Number(2.0)
                ]),
                Expression::List(vec![
                    Expression::Symbol("+".to_string()),
                    Expression::Number(2.0),
                    Expression::Number(5.0)
                ])
            ])
        ])]))
    );

    let mut env = Rc::new(RefCell::new(Environment::new()));
    if let Ok(res) = res {
        // will not pass, need refactor - todo
        assert_eq!(eval_exp(&res, &mut env), Ok(Expression::Number(11.0)))
    }
}
