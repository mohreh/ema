use std::{cell::RefCell, rc::Rc};

use ema::{environment::Environment, eval::eval_exp, parser::parse};

fn main() {
    let exp = parse(
        "
    (
        (def square (x) (
            (* x x)
        ))
    )",
    );
    let mut env = Rc::new(RefCell::new(Environment::new()));
    let res = eval_exp(&exp.unwrap(), &mut env);
    dbg!(res.unwrap());
}
