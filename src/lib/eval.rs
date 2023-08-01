use std::{cell::RefCell, cmp::Ordering, fs, rc::Rc};

use crate::{
    environment::Environment,
    error::Error,
    expression::{Expression, Object},
    parser::parse,
    transform::{
        transform_compound_assign, transform_def_to_var_lambda, transform_for_to_while,
        transform_incdec, transform_module_to_class, transform_switch_to_if,
    },
};

#[derive(Default, Debug)]
pub struct Evaluator {
    cwf_path: String,
    env_arena: Vec<Rc<RefCell<Environment>>>,
}

impl Evaluator {
    pub fn set_cwf_path(&mut self, cwf_path: String) {
        self.cwf_path = cwf_path;
    }

    pub fn eval_exp(
        &mut self,
        exp: &Expression,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        match exp {
            Expression::Void => Ok(Expression::Void),

            Expression::Number(num) => Ok(Expression::Number(*num)),

            Expression::String(str) => Ok(Expression::String(str.to_owned())),

            // access variable
            Expression::Symbol(str) => env.borrow_mut().lookup(str),

            Expression::List(list) => self.eval_list(list, env),

            Expression::Boolean(bool) => Ok(Expression::Boolean(*bool)),

            Expression::Function(params, body, env_idx) => Ok(Expression::Function(
                params.to_vec(),
                body.clone(),
                *env_idx,
            )),
            Expression::Object(obj) => Ok(Expression::Object(obj.clone())),
        }
    }

    fn eval_list(
        &mut self,
        list: &Vec<Expression>,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::*;

        if let Some(head) = list.get(0) {
            match head {
                Symbol(s) => match s.as_str() {
                    "+" | "-" | "*" | "/" | "%" | "<" | "<=" | ">" | ">=" | "=" | "!=" | "&"
                    | "|" => self.eval_binary_op(list, env),

                    "++" | "--" => self.eval_exp(&transform_incdec(list)?, env),

                    "+=" | "-=" | "*=" | "/=" | "%=" => {
                        self.eval_exp(&transform_compound_assign(list)?, env)
                    }
                    "var" => self.eval_define_variable(list, env),
                    "set" => self.eval_assign_variable(list, env),
                    "if" => self.eval_if(list, env),
                    "switch" => self.eval_exp(&transform_switch_to_if(list)?, env),
                    "while" => self.eval_while(list, env),
                    "for" => self.eval_exp(&transform_for_to_while(list)?, env),
                    "def" => self.eval_define_function(list, env),
                    "begin" => {
                        let mut nested_block_env =
                            Rc::new(RefCell::new(Environment::extend(env.clone())));

                        self.eval_block(list, &mut nested_block_env)
                    }
                    "lambda" => self.eval_define_lambda(list, env),
                    "class" => self.eval_define_class(list, env),
                    "new" => self.eval_new(list, env),
                    "prop" => self.eval_prop(list, env),
                    "super" => self.eval_super(list, env),
                    "module" => self.eval_module(list, env),
                    "import" => self.eval_import(list, env),
                    "print" => self.eval_print(list, env),
                    // user defined functions or variables
                    _ => {
                        if let Ok(Function(params, body, env_idx)) = self.eval_exp(head, env) {
                            // static scope
                            let mut activation_env = Rc::new(RefCell::new(Environment::extend(
                                self.env_arena
                                    .get(env_idx)
                                    .ok_or(Error::Reason("unexpected error".to_string()))?
                                    .clone(),
                            )));

                            return self.eval_function_body(
                                list,
                                params,
                                body,
                                env,
                                &mut activation_env,
                            );
                        }

                        self.eval_exp(head, env)
                    }
                },
                // immediately call function
                _ => {
                    let head_evaluated = self.eval_exp(head, env)?;
                    if let Expression::Function(params, body, env_idx) = head_evaluated.clone() {
                        match self.eval_function_body(
                            list,
                            params,
                            body,
                            env,
                            &mut Rc::new(RefCell::new(Environment::extend(
                                self.env_arena
                                    .get(env_idx)
                                    .ok_or(Error::Reason("unexpected error".to_string()))?
                                    .clone(),
                            ))),
                        ) {
                            Ok(val) => Ok(val),
                            Err(_) => Ok(head_evaluated),
                        }
                    } else {
                        Ok(head_evaluated)
                    }
                }
            }
        } else {
            Ok(Expression::Void)
        }
    }

    fn eval_import(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        if let Some((module_name, rest)) = list.split_last() {
            let module_file_path = match self.cwf_path.len() {
                0 => format!("./{}.eva", module_name),
                _ => format!("{}/{}.eva", self.cwf_path, module_name),
            };

            let ctx = fs::read_to_string(module_file_path)?;

            let body = if let Expression::List(body) = parse(&ctx)? {
                if body.len() > 1 {
                    return Err(Error::Reason("a module only contains one body".to_string()));
                }

                body[0].clone()
            } else {
                unreachable!()
            };

            match rest.len() {
                1 => self.eval_module(
                    &[
                        Expression::Symbol("module".to_string()),
                        module_name.clone(),
                        body,
                    ],
                    env,
                ),
                2 => {
                    let [_tag, import_names] = rest else {
                        unreachable!()
                    };

                    self.eval_module(
                        &[
                            Expression::Symbol("module".to_string()),
                            module_name.clone(),
                            body,
                        ],
                        env,
                    )?;

                    match import_names {
                        Expression::List(names) => {
                            let mut res = Expression::Void;
                            for name in names {
                                res = self.eval_exp(
                                    &Expression::List(vec![
                                        Expression::Symbol("var".to_string()),
                                        name.clone(),
                                        Expression::List(vec![
                                            Expression::Symbol("prop".to_string()),
                                            module_name.clone(),
                                            name.clone(),
                                        ]),
                                    ]),
                                    env,
                                )?;
                            }

                            Ok(res)
                        }

                        name => self.eval_exp(
                            &Expression::List(vec![
                                Expression::Symbol("var".to_string()),
                                name.clone(),
                                Expression::List(vec![
                                    Expression::Symbol("prop".to_string()),
                                    module_name.clone(),
                                    name.clone(),
                                ]),
                            ]),
                            env,
                        ),
                    }
                }
                _ => Err(Error::Reason("invalid import".to_string())),
            }
        } else {
            Err(Error::Reason("invalid import".to_string()))
        }
    }

    fn eval_module(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        self.eval_exp(&transform_module_to_class(list)?, env)
    }

    fn eval_define_class(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, name, parent, body] = &list else {
            return Err(Error::Invalid("invalid class/module definition".to_string()))
        };

        let name = match name {
            Expression::Symbol(name) => name.clone(),
            _ => return Err(Error::Invalid("invalid class/module name".to_string())),
        };

        let mut parent_idx: Option<_> = None; // parent class

        // parent environment
        let parent_env = match self.eval_exp(parent, env)? {
            Expression::Object(obj) => {
                parent_idx = Some(Rc::new(RefCell::new(obj.clone())));

                self.env_arena.get(obj.idx).ok_or(Error::Reason(format!(
                    "cannot get parent for class {}",
                    name
                )))?
            }
            Expression::Void => env,
            _ => {
                return Err(Error::Invalid(format!(
                    "parent of class {} has invalid type",
                    name
                )))
            }
        };

        let mut class_env = Rc::new(RefCell::new(Environment::extend(parent_env.clone())));

        if let Expression::List(body_list) = body {
            match &body_list[0] {
                Expression::Symbol(sym) if sym == &"begin".to_string() => {
                    self.eval_block(body_list, &mut class_env)?;
                    // self.eval_exp(&Expression::List(body_list[1..].to_vec()), &mut class_env)?;
                }
                _ => {
                    self.eval_exp(&Expression::List(body_list.to_vec()), &mut class_env)?;
                }
            }
        } else {
            self.eval_exp(body, &mut class_env)?;
        }

        self.env_arena.push(class_env.clone());

        env.borrow_mut().define(
            &name,
            Expression::Object(Object {
                idx: self.env_arena.len() - 1,
                parent: parent_idx,
            }),
        )
    }

    fn eval_new(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let (class_name, rest) = &list[1..].split_first().ok_or(Error::Invalid(
            "invalid creating new instance of class".to_string(),
        ))?;

        if let Expression::Object(obj) = self.eval_exp(class_name, env)? {
            let class_env = self.env_arena.get(obj.idx).unwrap();
            let mut instance_env = Rc::new(RefCell::new(Environment::extend(class_env.clone())));

            let constructor_fn = instance_env.borrow_mut().lookup("constructor")?;
            if let Expression::Function(params, body, env_idx) = constructor_fn {
                let mut rest = rest.to_vec();

                rest.insert(0, Expression::Symbol("constructor".to_string())); // function name
                rest.insert(
                    1,
                    Expression::Object(Object {
                        idx: env_idx,
                        parent: None,
                    }),
                ); // passing self

                self.eval_function_body(&rest, params, body, env, &mut instance_env)?;

                self.env_arena.push(instance_env);
                Ok(Expression::Object(Object {
                    idx: self.env_arena.len() - 1,
                    parent: obj.parent,
                }))
            } else {
                Err(Error::Reason(
                    "cannot get valid constructor for class".to_string(),
                ))
            }
        } else {
            Err(Error::Invalid(
                "invalid creating new instance of class".to_string(),
            ))
        }
    }

    fn eval_prop(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, instance, name] = list else {
            return Err(Error::Invalid("invalid access to class properties".to_string()));
        };

        let name = match name {
            Expression::Symbol(name) => name.clone(),
            _ => return Err(Error::Invalid("invalid property name".to_string())),
        };

        if let Expression::Object(obj) = self.eval_exp(instance, env)? {
            let instance_env = self.env_arena.get_mut(obj.idx).unwrap();
            instance_env.borrow_mut().lookup(&name)
        } else {
            Err(Error::Reason(format!(
                "{} is not a instance of a class",
                instance
            )))
        }
    }

    fn eval_super(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, class_name] = list else {
            return Err(Error::Invalid("invalid super call".to_string()));
        };

        if let Expression::Object(obj) = self.eval_exp(class_name, env)? {
            Ok(Expression::Object(
                obj.parent
                    .ok_or(Error::Reason("cannot find parent".to_string()))?
                    .borrow()
                    .clone(),
            ))
        } else {
            Err(Error::Invalid(
                "invalid super call on non class".to_string(),
            ))
        }
    }

    // block: sequence of expression
    fn eval_block(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let mut result: Expression = Expression::Void;

        if let Some((_tag, rest)) = list.split_first() {
            for exp in rest {
                result = self.eval_exp(exp, env)?;
            }
        }

        Ok(result)
    }

    fn eval_while(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, condition, body] = &list else {
            return Err(Error::Invalid("invalid while statement".to_string()))
        };

        let mut result = Expression::Void;
        loop {
            if let Expression::Boolean(cond) = self.eval_exp(condition, env)? {
                if cond {
                    result = self.eval_exp(body, env)?;
                } else {
                    break;
                }
            } else {
                return Err(Error::Invalid("invalid while statement".to_string()));
            }
        }

        Ok(result)
    }

    fn eval_if(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, condition, consequent, alternate] = &list else {
        return Err(Error::Invalid("invalid if statement".to_string()))
    };

        if let Expression::Boolean(cond) = self.eval_exp(condition, env)? {
            if cond {
                self.eval_exp(consequent, env)
            } else {
                self.eval_exp(alternate, env)
            }
        } else {
            Err(Error::Invalid("invalid if statement".to_string()))
        }
    }

    fn eval_define_function(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        // JIT-transpile to a variable declaration
        self.eval_exp(&transform_def_to_var_lambda(list)?, env)
    }

    fn eval_define_lambda(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, params, body] = &list else {
        return Err(Error::Invalid("invalid defining lambda.".to_string()))
    };

        let params = {
            match params {
                Expression::List(list) => list
                    .iter()
                    .map(|param| match param {
                        Expression::Symbol(name) => Ok(name.clone()),
                        _ => Err(Error::Invalid("invalid params for lambda".to_string())),
                    })
                    .collect::<Result<Vec<String>, Error>>()?,
                Expression::Symbol(name) => vec![name.clone()],
                _ => return Err(Error::Invalid("invalid params for lambda".to_string())),
            }
        };

        self.env_arena.push(env.clone());

        Ok(Expression::Function(
            params,
            Rc::new(RefCell::new(body.clone())),
            self.env_arena.len() - 1,
        ))
    }

    fn eval_function_body(
        &mut self,
        list: &[Expression],
        params: Vec<String>,
        body: Rc<RefCell<Expression>>,
        env: &mut Rc<RefCell<Environment>>,
        activation_env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        if let Some((func_name, args)) = list.split_first() {
            if params.len() != args.len() {
                return Err(Error::Reason(format!(
                    "function {} took {} argurments you provide {}",
                    func_name,
                    params.len(),
                    args.len()
                )));
            }

            for (idx, param_name) in params.iter().enumerate() {
                activation_env.borrow_mut().define(
                    param_name,
                    self.eval_exp(
                        args.get(idx)
                            .ok_or(Error::Reason("unexpected error".to_string()))?,
                        env,
                    )?,
                )?;
            }

            self.eval_exp(&body.borrow(), activation_env)
        } else {
            Err(Error::Reason("unexpected error".to_string()))
        }
    }

    fn eval_define_variable(
        &mut self,
        list: &Vec<Expression>,
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::Symbol;

        if list.len() != 3 {
            return Err(Error::Invalid("Invalid number of argurments".to_string()));
        }

        if let Symbol(name) = &list[1] {
            let value = self.eval_exp(&list[2], env)?;
            env.borrow_mut().define(name, value)
        } else {
            Err(Error::Invalid("Invalid defining variable".to_string()))
        }
    }

    fn eval_assign_variable(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        let [_tag, reference, value] = &list else {
            return Err(Error::Invalid("invalid set statement".to_string()));
        };

        match reference {
            Expression::List(exp_list) => match &exp_list[0] {
                Expression::Symbol(sym) if sym == &"prop".to_string() => {
                    let [_tag, instance, prop_name] = &exp_list[..] else {
                        return Err(Error::Invalid("invalid access to class properties".to_string()));
                    };

                    let prop_name = match prop_name {
                        Expression::Symbol(name) => name.clone(),
                        _ => return Err(Error::Invalid("invalid property name".to_string())),
                    };

                    let value = self.eval_exp(value, env)?;

                    if let Expression::Object(obj) = self.eval_exp(instance, env)? {
                        let instance_env = self.env_arena.get_mut(obj.idx).unwrap();
                        instance_env.borrow_mut().define(&prop_name, value)
                    } else {
                        Err(Error::Reason(format!(
                            "{} is not a instance of a class",
                            instance
                        )))
                    }
                }
                _ => Err(Error::Invalid("Invalid assigning variable".to_string())),
            },
            Expression::Symbol(name) => {
                let value = self.eval_exp(value, env)?;
                env.borrow_mut().assign(name, value)
            }
            _ => Err(Error::Invalid("Invalid assigning variable".to_string())),
        }
    }

    fn eval_print(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        if let Some((_, args)) = list.split_first() {
            for arg in args {
                let exp = self.eval_exp(arg, env)?;
                print!("{}", exp)
            }
            println!();

            Ok(Expression::Void)
        } else {
            Err(Error::Reason("unexpected error.".to_string()))
        }
    }

    fn eval_binary_op(
        &mut self,
        list: &[Expression],
        env: &mut Rc<RefCell<Environment>>,
    ) -> Result<Expression, Error> {
        use Expression::*;

        let head = &list[0];
        let left = self.eval_exp(&list[1], env)?;
        let right = self.eval_exp(&list[2], env)?;

        match head {
            Expression::Symbol(str) => match str.as_str() {
                "+" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val + right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } + right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val + if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } + if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => Ok(String(left_val + &right_val)),
                    _ => Err(Error::Invalid("invalid type for + operator".to_string())),
                },
                "-" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val - right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } - right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val - if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } - if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for - operator".to_string())),
                },
                "*" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val * right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } * right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val * if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } * if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for * operator".to_string())),
                },

                "/" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val / right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } / right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val / if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } / if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for / operator".to_string())),
                },
                "%" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Number(left_val % right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Number(if left_val { 1.0 } else { 0.0 } % right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Number(left_val % if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Number(
                        if left_val { 1.0 } else { 0.0 } % if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for % operator".to_string())),
                },
                ">" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val > right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } > right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val > if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } > if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Greater))
                    }
                    _ => Err(Error::Invalid("invalid type for > operator".to_string())),
                },

                ">=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val >= right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } >= right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val >= if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } >= if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for >= operator".to_string())),
                },
                "<" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val < right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } < right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val < if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } < if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Less))
                    }
                    _ => Err(Error::Invalid("invalid type for < operator".to_string())),
                },

                "<=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val <= right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } <= right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val <= if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } <= if right_val { 1.0 } else { 0.0 },
                    )),
                    _ => Err(Error::Invalid("invalid type for <= operator".to_string())),
                },
                "=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val == right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } == right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val == if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } == if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) == Ordering::Equal))
                    }
                    _ => Err(Error::Invalid("invalid type for == operator".to_string())),
                },

                "!=" => match (left, right) {
                    (Number(left_val), Number(right_val)) => Ok(Boolean(left_val != right_val)),
                    (Boolean(left_val), Number(right_val)) => {
                        Ok(Boolean(if left_val { 1.0 } else { 0.0 } != right_val))
                    }
                    (Number(left_val), Boolean(right_val)) => {
                        Ok(Boolean(left_val != if right_val { 1.0 } else { 0.0 }))
                    }
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(
                        if left_val { 1.0 } else { 0.0 } != if right_val { 1.0 } else { 0.0 },
                    )),
                    (String(left_val), String(right_val)) => {
                        Ok(Boolean(left_val.cmp(&right_val) != Ordering::Equal))
                    }
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },

                "&" => match (left, right) {
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(left_val & right_val)),
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },

                "|" => match (left, right) {
                    (Boolean(left_val), Boolean(right_val)) => Ok(Boolean(left_val | right_val)),
                    _ => Err(Error::Invalid("invalid type for != operator".to_string())),
                },
                _ => todo!(),
            },
            _ => Err(Error::Invalid("invalid operator".to_string())),
        }
    }
}
