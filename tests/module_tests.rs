use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn define_and_use_class() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
(begin
    (module Math
        (begin
            (def abs (val)
                (if (< val 0)
                    (- 0 val) 
                    val 
                ) 
            )
        
            (def square (x) (* x x))
        
            (var MAX_VAL 1000)
        )
    )

    (begin 
        (var x ((prop Math abs) -10))
        (var abs (prop Math abs))
        (var square (prop Math square))
        (var y (square (abs -20)))
        (- (prop Math MAX_VAL) (+ x y))
    )
)
",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(590.0))
    );
}
