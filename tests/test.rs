use ema::{expression::Expression, literal::Literal, Ema};

#[test]
fn self_evaluate_exp() {
    assert_eq!(
        Ema::eval(Expression::Literal(Literal::Number(1.0))),
        Ok(Literal::Number(1.0))
    );

    let string = "'hello'".to_string();
    assert_eq!(
        Ema::eval(Expression::Literal(Literal::String(string.clone()))),
        Ok(Literal::String(string.clone()))
    );
}
