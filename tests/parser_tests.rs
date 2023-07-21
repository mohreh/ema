use std::{cell::RefCell, rc::Rc};

use ema::{
    environment::Environment, error::Error, eval::eval_exp, expression::Expression, parser::parse,
};

#[test]
fn parse_code() {
    let res = parse("(var x (+ (* 2 2) (+ 2 5)))");
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
        assert_eq!(eval_exp(&res, &mut env), Ok(Expression::Number(11.0)))
    }
}

#[test]
fn should_return_err_unbalanced_parens() {
    assert_eq!(parse(")"), Err(Error::Parse("unexpected ')'".to_string())));

    assert_eq!(
        parse("("),
        Err(Error::Parse("could not find closing ')'".to_string()))
    );

    assert_eq!(
        parse("(()"),
        Err(Error::Parse("could not find closing ')'".to_string()))
    );

    assert_eq!(
        parse("())"),
        Err(Error::Parse("unexpected ')'".to_string()))
    );

    assert_eq!(parse(")("), Err(Error::Parse("unexpected ')'".to_string())));

    assert_eq!(
        parse("()"),
        Ok(Expression::List(vec![Expression::List(vec![])]))
    );
}
