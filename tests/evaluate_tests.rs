use std::{cell::RefCell, collections::HashMap, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression};

use Expression::*;
#[test]
fn self_evaluate_expression() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    assert_eq!(eval.eval_exp(&Number(1.0), &mut env), Ok(Number(1.0)));

    assert_eq!(
        eval.eval_exp(&String("hello".to_string()), &mut env),
        Ok(String("hello".to_string())),
    );

    assert_eq!(
        eval.eval_exp(
            &List(vec![Symbol("+".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(7.0)),
    );
}

#[test]
fn math_operation() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    // (+ 2 5) = 7
    assert_eq!(
        eval.eval_exp(
            &List(vec![Symbol("+".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(7.0)),
    );

    // (+ (+ 5 5) 5) = 15
    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("+".to_string()),
                List(vec![Symbol("+".to_string()), Number(5.0), Number(5.0),]),
                Number(5.0),
            ]),
            &mut env
        ),
        Ok(Number(15.0)),
    );

    // (+ (+ 5 (+ 5 5)) 5) = 20
    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("+".to_string()),
                List(vec![
                    Symbol("+".to_string()),
                    List(vec![Symbol("+".to_string()), Number(5.0), Number(5.0),]),
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
        eval.eval_exp(
            &List(vec![Symbol("-".to_string()), Number(2.0), Number(5.0),]),
            &mut env
        ),
        Ok(Number(-3.0)),
    );
}

#[test]
fn define_and_access_variable() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("var".to_string()),
                Symbol("x".to_string()),
                Number(5.0),
            ]),
            &mut env,
        ),
        Ok(Number(5.0))
    );

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("var".to_string()),
                Symbol("s".to_string()),
                List(vec![
                    Symbol("-".to_string()),
                    Symbol("x".to_string()),
                    Number(1.0),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(4.0))
    );

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("var".to_string()),
                Symbol("y".to_string()),
                List(vec![Symbol("-".to_string()), Number(2.0), Number(5.0),]),
            ]),
            &mut env,
        ),
        Ok(Number(-3.0))
    );

    assert_eq!(
        eval.eval_exp(
            &List(vec![Symbol("var".to_string()), Number(5.0), Number(2.0)]),
            &mut env,
        ),
        Err(ema::error::Error::Invalid(
            "Invalid defining variable".to_string()
        ))
    );

    assert_eq!(
        env.borrow_mut().record,
        HashMap::from([
            ("x".to_string(), Number(5.0)),
            ("y".to_string(), Number(-3.0)),
            ("s".to_string(), Number(4.0)),
            // predefined variables
            ("nil".to_string(), Void),
            ("true".to_string(), Expression::Boolean(true)),
            ("false".to_string(), Expression::Boolean(false)),
        ])
    );

    // access variable
    assert_eq!(
        eval.eval_exp(&Symbol("x".to_string()), &mut env),
        Ok(Number(5.0))
    );

    assert_eq!(
        eval.eval_exp(&Symbol("z".to_string()), &mut env),
        Err(ema::error::Error::Reference("z is not defined".to_string()))
    )
}

#[test]
fn test_predefined_vars() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::from(HashMap::from([
        ("true".to_string(), Boolean(true)),
        ("false".to_string(), Boolean(false)),
        // ("null".to_string(), Option::None),
    ]))));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("var".to_string()),
                Symbol("x".to_string()),
                Boolean(true)
            ]),
            &mut env,
        ),
        Ok(Boolean(true))
    );

    assert_eq!(
        eval.eval_exp(&Symbol("x".to_string()), &mut env),
        Ok(Boolean(true))
    );
}

#[test]
fn block_expression() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("x".to_string()),
                    Number(20.0)
                ]),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("y".to_string()),
                    Number(50.0)
                ]),
                List(vec![
                    Symbol("+".to_string()),
                    List(vec![
                        Symbol("*".to_string()),
                        Symbol("x".to_string()),
                        Symbol("y".to_string()),
                    ]),
                    Number(40.0),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(1040.0))
    );

    // an empty block should return void
    assert_eq!(
        eval.eval_exp(&List(vec![]), &mut env,),
        Ok(Expression::Void)
    );
}

#[test]
fn nested_env_should_not_affect_outer() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    // (
    //     (var x 10)
    //     (
    //         (var x 20)
    //         x
    //     )
    //     x
    // ) => 10
    assert_eq!(
        eval.eval_exp(
            &List(vec![
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("x".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    List(vec![
                        Symbol("var".to_string()),
                        Symbol("x".to_string()),
                        Number(20.0)
                    ]),
                    Symbol("x".to_string())
                ]),
                Symbol("x".to_string())
            ]),
            &mut env,
        ),
        Ok(Number(10.0))
    )
}

#[test]
fn access_variable_from_outer_env() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::from(HashMap::from([(
        "global_var".to_string(),
        Number(10.0),
    )]))));

    // (begin
    //      (var outer 10)
    //      (var result (begin
    //          (var inner (+ outer global_var))
    //          inner
    //      ))
    //      result
    // ) = 20
    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("outer".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("outer_2".to_string()),
                    Number(15.0)
                ]),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("result".to_string()),
                    List(vec![
                        Symbol("begin".to_string()),
                        List(vec![
                            Symbol("var".to_string()),
                            Symbol("inner".to_string()),
                            List(vec![
                                Symbol("+".to_string()),
                                Symbol("global_var".to_string()),
                                Symbol("outer".to_string()),
                            ]),
                        ]),
                        Symbol("inner".to_string())
                    ]),
                ]),
                List(vec![
                    Symbol("begin".to_string()),
                    List(vec![
                        Symbol("var".to_string()),
                        Symbol("inner".to_string()),
                        List(vec![
                            Symbol("+".to_string()),
                            Symbol("outer".to_string()),
                            Number(10.0),
                        ]),
                    ]),
                    Symbol("inner".to_string())
                ]),
                Symbol("result".to_string())
            ]),
            &mut env,
        ),
        Ok(Number(20.0))
    )
}

#[test]
fn assign_new_value_to_outer_variable() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    // (begin
    //      (var outer 10)
    //      (begin
    //          (set outer 20)
    //          outer
    //      )
    // ) = 20
    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("outer".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    Symbol("begin".to_string()),
                    List(vec![
                        Symbol("set".to_string()),
                        Symbol("outer".to_string()),
                        Number(20.0)
                    ]),
                    Symbol("outer".to_string())
                ])
            ]),
            &mut env,
        ),
        Ok(Number(20.0))
    );

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("outer".to_string()),
                    Number(10.0)
                ]),
                List(vec![
                    Symbol("begin".to_string()),
                    List(vec![
                        Symbol("set".to_string()),
                        Symbol("outer".to_string()),
                        Number(20.0)
                    ]),
                    List(vec![Symbol("outer".to_string()),]),
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
    let mut eval = Evaluator::default();
    // (begin
    //     (var x 10)
    //     (var y 0)
    //     (if (> x 10)
    //          (set y 20)
    //          (set y 30)
    //     )
    //     y
    // )
    let mut env = Rc::new(RefCell::new(Environment::new()));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("x".to_string()),
                    Number(10.0),
                ]),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("y".to_string()),
                    Number(0.0),
                ]),
                List(vec![
                    Symbol("if".to_string()),
                    List(vec![
                        Symbol(">".to_string()),
                        Symbol("x".to_string()),
                        Number(10.0),
                    ]),
                    List(vec![
                        Symbol("set".to_string()),
                        Symbol("y".to_string()),
                        Number(20.0),
                    ]),
                    List(vec![
                        Symbol("set".to_string()),
                        Symbol("y".to_string()),
                        Number(30.0),
                    ]),
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(30.0))
    );
}

// (while <condition>
//     <expression>
// )
#[test]
fn while_control_flow() {
    let mut eval = Evaluator::default();
    // (begin
    //     (var counter 0)
    //     (var result 0)
    //     (while (< counter 10)
    //          (begin
    //              (set result (+ result 2))
    //              (set counter (+ counter 1))
    //          )
    //     )
    //     y
    // )
    let mut env = Rc::new(RefCell::new(Environment::new()));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("begin".to_string()),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("counter".to_string()),
                    Number(0.0),
                ]),
                List(vec![
                    Symbol("var".to_string()),
                    Symbol("result".to_string()),
                    Number(0.0),
                ]),
                List(vec![
                    Symbol("while".to_string()),
                    List(vec![
                        Symbol("<".to_string()),
                        Symbol("counter".to_string()),
                        Number(10.0),
                    ]),
                    List(vec![
                        Symbol("begin".to_string()),
                        List(vec![
                            Symbol("set".to_string()),
                            Symbol("result".to_string()),
                            List(vec![
                                Symbol("+".to_string()),
                                Symbol("result".to_string()),
                                Number(20.0),
                            ]),
                        ]),
                        List(vec![
                            Symbol("set".to_string()),
                            Symbol("counter".to_string()),
                            List(vec![
                                Symbol("+".to_string()),
                                Symbol("counter".to_string()),
                                Number(1.0),
                            ]),
                        ]),
                        Symbol("result".to_string())
                    ])
                ]),
            ]),
            &mut env,
        ),
        Ok(Number(200.0))
    );
}

#[test]
fn sum_op_for_string() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    assert_eq!(
        eval.eval_exp(
            &List(vec![
                Symbol("+".to_string()),
                String("Hello".to_string()),
                String("World".to_string()),
            ]),
            &mut env
        ),
        Ok(String("HelloWorld".to_string())),
    );
}
