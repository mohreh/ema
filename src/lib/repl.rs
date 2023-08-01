use std::{cell::RefCell, io::stdin, rc::Rc};

use linefeed::{Interface, ReadResult};

use crate::{
    environment::Environment, error::Error, eval::Evaluator, expression::Expression, parser::parse,
};

const PROMPT: &str = "ema> ";

pub fn repl() {
    let reader = Interface::new(PROMPT).unwrap();
    let mut eval = Evaluator::default();
    let mut global_env = Rc::new(RefCell::new(Environment::default()));

    reader.set_prompt(PROMPT).unwrap();
    while let ReadResult::Input(mut input) = reader.read_line().unwrap() {
        if input.eq("exit") {
            break;
        }

        match evaluate_input(&mut input, &mut eval, &mut global_env) {
            Ok(val) => println!("{}", val),
            Err(err) => println!("{}", err),
        };
    }

    println!("good bye");
}

fn evaluate_input(
    input: &mut String,
    eval: &mut Evaluator,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Expression, Error> {
    match parse(input) {
        Ok(exp) => eval.eval_exp(&exp, env),
        Err(err) => match err {
            Error::Parse(err_desc) if err_desc == *"could not find closing ')'" => {
                let mut buf = String::new();
                loop {
                    stdin().read_line(&mut buf).unwrap();

                    if buf.contains("\n\n") {
                        input.push_str(&buf);
                        return evaluate_input(input, eval, env);
                    }
                }
            }
            _ => Err(err),
        },
    }
}
