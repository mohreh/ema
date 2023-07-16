use ema::{environment::Environment, eval_exp, expression::Expression};

use Expression::*;
#[test]
fn self_evaluate_exp() {
    let mut env = Environment::new();
    assert_eq!(eval_exp(&Number(1.0), &mut env), Ok(Number(1.0)));

    assert_eq!(
        eval_exp(&String("'hello'".to_string()), &mut env),
        Ok(String("hello".to_string())),
    );

    assert_eq!(
        eval_exp(
            &List(vec![String("+".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(7.0)),
    );
}

#[test]
fn math_op() {
    let mut env = Environment::new();

    // (+ 2 5) = 7
    assert_eq!(
        eval_exp(
            &List(vec![String("+".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(7.0)),
    );

    // (+ (+ 5 5) 5) = 15
    assert_eq!(
        eval_exp(
            &List(vec![
                String("+".to_string()),
                List(vec![String("+".to_string()), Number(5.0), Number(5.0),]),
                Number(5.0),
            ]),
            &mut env
        ),
        Ok(Number(15.0)),
    );

    // (+ (+ 5 (+ 5 5)) 5) = 20
    assert_eq!(
        eval_exp(
            &List(vec![
                String("+".to_string()),
                List(vec![
                    String("+".to_string()),
                    List(vec![String("+".to_string()), Number(5.0), Number(5.0),]),
                    Number(5.0),
                ]),
                Number(5.0),
            ]),
            &mut env
        ),
        Ok(Number(20.0)),
    );

    // (- 2 5)
    assert_eq!(
        eval_exp(
            &List(vec![String("-".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(-3.0)),
    );
}

// #[test]
// fn sum_op_for_string() {
//     assert_eq!(
//         eval_exp(&List(vec![
//             String("+".to_string()),
//             String("'Hello'".to_string()),
//             String("'World'".to_string()),
//         ])),
//         Ok(String("HelloWorld".to_string())),
//     );
// }
