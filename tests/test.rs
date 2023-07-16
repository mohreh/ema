use ema::{eval_exp, expression::Expression};

#[test]
fn self_evaluate_exp() {
    use Expression::*;
    assert_eq!(eval_exp(Number(1.0)), Ok(Number(1.0)));

    assert_eq!(
        eval_exp(String("'hello'".to_string())),
        Ok(String("hello".to_string())),
    );

    assert_eq!(
        eval_exp(List(vec![
            String("+".to_string()),
            Number(2.0),
            Number(5.0),
        ])),
        Ok(Number(7.0)),
    );
}
