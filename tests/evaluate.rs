use std::collections::HashMap;

use ema::{environment::Environment, eval_exp, expression::Expression};

use Expression::*;
#[test]
fn self_evaluate_expression() {
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
fn math_operation() {
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

#[test]
fn define_and_access_variable() {
    let mut env = Environment::new();

    assert_eq!(
        eval_exp(
            &List(vec![
                String("var".to_string()),
                String("x".to_string()),
                Number(5.0),
            ]),
            &mut env,
        ),
        Ok(Number(5.0))
    );

    assert_eq!(
        eval_exp(
            &List(vec![
                String("var".to_string()),
                String("s".to_string()),
                List(vec![
                    String("-".to_string()),
                    String("x".to_string()),
                    Number(1.0),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(4.0))
    );

    assert_eq!(
        eval_exp(
            &List(vec![
                String("var".to_string()),
                String("y".to_string()),
                List(vec![String("-".to_string()), Number(2.0), Number(5.0),]),
            ]),
            &mut env,
        ),
        Ok(Number(-3.0))
    );

    assert_eq!(
        eval_exp(
            &List(vec![String("var".to_string()), Number(5.0), Number(2.0)]),
            &mut env,
        ),
        Err(ema::error::Error::Reason(
            "Invalid defining variable".to_string()
        ))
    );

    assert_eq!(
        env.record,
        HashMap::from([
            ("x".to_string(), Number(5.0)),
            ("y".to_string(), Number(-3.0)),
            ("s".to_string(), Number(4.0))
        ])
    );

    // access variable
    assert_eq!(
        eval_exp(&String("x".to_string()), &mut env),
        Ok(Number(5.0))
    );

    assert_eq!(
        eval_exp(&String("z".to_string()), &mut env),
        Err(ema::error::Error::Reference(
            "variable z is not defined".to_string()
        ))
    )
}

#[test]
fn test_predefined_vars() {
    let mut env = Environment::from(HashMap::from([
        ("true".to_string(), Boolean(true)),
        ("false".to_string(), Boolean(false)),
        // ("null".to_string(), Option::None),
    ]));

    assert_eq!(
        eval_exp(
            &List(vec![
                String("var".to_string()),
                String("x".to_string()),
                Boolean(true)
            ]),
            &mut env,
        ),
        Ok(Boolean(true))
    );

    assert_eq!(
        eval_exp(&String("x".to_string()), &mut env),
        Ok(Boolean(true))
    );
}

#[test]
fn block_expression() {
    let mut env = Environment::new();

    assert_eq!(
        eval_exp(
            &List(vec![
                List(vec![
                    String("var".to_string()),
                    String("x".to_string()),
                    Number(20.0)
                ]),
                List(vec![
                    String("var".to_string()),
                    String("y".to_string()),
                    Number(50.0)
                ]),
                List(vec![
                    String("+".to_string()),
                    List(vec![
                        String("*".to_string()),
                        String("x".to_string()),
                        String("y".to_string()),
                    ]),
                    Number(40.0),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(1040.0))
    );

    // an empty block should return false: () = false
    assert_eq!(eval_exp(&List(vec![]), &mut env,), Ok(Boolean(false)));
}

#[test]
fn nested_env_should_not_affect_outer() {
    let mut env = Environment::new();

    // (
    //     (var x 10)
    //     (
    //         (var x 20)
    //         x
    //     )
    //     x
    // ) => 10
    assert_eq!(
        eval_exp(
            &List(vec![
                List(vec![
                    String("var".to_string()),
                    String("x".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    List(vec![
                        String("var".to_string()),
                        String("x".to_string()),
                        Number(20.0)
                    ]),
                    String("x".to_string())
                ]),
                String("x".to_string())
            ]),
            &mut env,
        ),
        Ok(Number(10.0))
    )
}

#[test]
fn access_variable_from_outer_env() {
    let mut env = Environment::from(HashMap::from([("global_var".to_string(), Number(10.0))]));

    // (
    //      (var outer 10)
    //      (var result (
    //          (var inner (+ outer global_var))
    //          inner
    //      ))
    //      result
    // ) = 20
    assert_eq!(
        eval_exp(
            &List(vec![
                List(vec![
                    String("var".to_string()),
                    String("outer".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    String("var".to_string()),
                    String("outer_2".to_string()),
                    Number(15.0)
                ]),
                List(vec![
                    String("var".to_string()),
                    String("result".to_string()),
                    List(vec![
                        List(vec![
                            String("var".to_string()),
                            String("inner".to_string()),
                            List(vec![
                                String("+".to_string()),
                                String("global_var".to_string()),
                                String("outer".to_string()),
                            ]),
                        ]),
                        String("inner".to_string())
                    ]),
                ]),
                List(vec![
                    List(vec![
                        String("var".to_string()),
                        String("inner".to_string()),
                        List(vec![
                            String("+".to_string()),
                            String("outer".to_string()),
                            Number(10.0),
                        ]),
                    ]),
                    String("inner".to_string())
                ]),
                String("result".to_string())
            ]),
            &mut env,
        ),
        Ok(Number(20.0))
    )
}

#[test]
fn assign_new_value_to_outer_variable() {
    let mut env = Environment::new();

    // (
    //      (var outer 10)
    //      (
    //          (set outer 20)
    //          outer
    //      )
    // ) = 20
    assert_eq!(
        eval_exp(
            &List(vec![
                List(vec![
                    String("var".to_string()),
                    String("outer".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    List(vec![
                        String("set".to_string()),
                        String("outer".to_string()),
                        Number(20.0)
                    ]),
                    List(vec![String("outer".to_string()),])
                ])
            ]),
            &mut env,
        ),
        Ok(Number(20.0))
    )
}

// (if <condition>
//     <consequent>
//     <alternative>
// )
#[test]
fn if_control_flow() {
    // (
    //     (var x 10)
    //     (var y 0)
    //     (if (> x 10)
    //          (set y 20)
    //          (set y 30)
    //     )
    //     y
    // )
    let mut env = Environment::new();

    assert_eq!(
        eval_exp(
            &List(vec![
                List(vec![
                    String("var".to_string()),
                    String("x".to_string()),
                    Number(10.0),
                ]),
                List(vec![
                    String("var".to_string()),
                    String("y".to_string()),
                    Number(0.0),
                ]),
                List(vec![
                    String("if".to_string()),
                    List(vec![
                        String(">".to_string()),
                        String("x".to_string()),
                        Number(10.0),
                    ]),
                    List(vec![
                        String("set".to_string()),
                        String("y".to_string()),
                        Number(20.0),
                    ]),
                    List(vec![
                        String("set".to_string()),
                        String("y".to_string()),
                        Number(30.0),
                    ]),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(30.0))
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
