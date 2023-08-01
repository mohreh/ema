use std::{cell::RefCell, fs, rc::Rc};

use crate::{environment::Environment, eval::Evaluator, parser::parse};

pub mod environment;
pub mod error;
pub mod eval;
pub mod expression;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod transform;

pub fn run_code(path: String) {
    let mut eval = Evaluator::default();
    let mut env = Rc::new(RefCell::new(Environment::new()));

    match fs::read_to_string(path) {
        Ok(ctx) => {
            match parse(&ctx) {
                Ok(exp) => match eval.eval_exp(&exp, &mut env) {
                    Ok(_) => (),
                    Err(err) => println!("{}", err),
                },
                Err(err) => println!("{}", err),
            };
        }
        Err(err) => println!("{}", err),
    }
}
