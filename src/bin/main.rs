use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::Evaluator, parser::parse};

fn main() {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let exp = parse(
        "
                (def constructor (self x y)
                    (begin
                        (set (prop self x) x) 
                        (set (prop self y) y) 
                    ) 
                )

    ",
    );

    let _ = eval.eval_exp(&exp.unwrap(), &mut env);

    dbg!(&env);
}
