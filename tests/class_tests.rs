use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, expression::Expression, parser::parse};

#[test]
fn define_and_use_class() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (class Point nil 
            (begin
                (def constructor (self x y)
                    (begin
                        (set (prop self x) x) 
                        (set (prop self y) y) 
                    ) 
                )


                (def calc (self)
                    (+
                        (prop self x) 
                        (prop self y) 
                    ) 
                )
            )
        )

        (var p (new Point 10 20))
        ((prop p calc) p)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(30.0))
    );
}

#[test]
fn inheritance() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
    (begin
        (class Point nil 
            (begin
                (def constructor (self x y)
                    (begin
                        (set (prop self x) x) 
                        (set (prop self y) y) 
                    ) 
                )


                (def calc (self)
                    (+
                        (prop self x) 
                        (prop self y) 
                    ) 
                )
            )
        )

        (class Point3D Point 
            (begin
                (def constructor (self x y z)
                    (begin
                        ((prop (super Point3D) constructor) x y) 
                        (set (prop self y) y) 
                    ) 
                )


                (def calc (self)
                    (+
                        ((prop (super Point3D) calc) self) 
                        (prop self z) 
                    ) 
                )
            )
        )

        (var p (new Point3D 10 20 30))
        ((prop p calc) p)
    )",
    );

    assert_eq!(
        eval.eval_exp(&exp.unwrap(), &mut env),
        Ok(Expression::Number(60.0))
    );
}
