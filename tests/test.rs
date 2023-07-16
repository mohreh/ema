use ema::{eval, expression::Expression, literal::Literal};

#[test]
fn self_evaluate_exp() {
    assert_eq!(
        eval(Expression::Literal(Literal::Number(1.0))),
        Ok(Literal::Number(1.0))
    );

    assert_eq!(
        eval(Expression::Literal(Literal::String("'hello'".to_string()))),
        Ok(Literal::String("hello".to_string())),
    );
}
