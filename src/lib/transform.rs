use crate::{error::Error, expression::Expression};

pub fn transform_def_to_var_lambda(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, name, params, body] = &list else {
            return Err(Error::Invalid("invalid defining function.".to_string()))
        };

    let name = match name {
        Expression::Symbol(name) => name.clone(),
        _ => return Err(Error::Invalid("invalid function name".to_string())),
    };

    Ok(Expression::List(vec![
        Expression::Symbol("var".to_owned()),
        Expression::Symbol(name),
        Expression::List(vec![
            Expression::Symbol("lambda".to_string()),
            params.clone(),
            body.clone(),
        ]),
    ]))
}

pub fn transform_switch_to_if(list: &[Expression]) -> Result<Expression, Error> {
    if let Some((_tag, cases)) = list.split_first() {
        let cases = cases.iter().rev().collect::<Vec<&Expression>>();

        if cases.len() < 2 {
            return Err(Error::Invalid(
                "switch statement must have at least 1 condition followed by else statement"
                    .to_string(),
            ));
        }

        let (last_one, rest) = cases
            .split_first()
            .ok_or(Error::Reason("switch is empty".to_string()))?;

        let mut result = match last_one {
            Expression::List(alt_list) => {
                let [tag, block] = &alt_list[..] else {
                    return Err(Error::Invalid(
                        "else case of switch statement must followed by 1 block expression".to_string(),
                    ));
                };
                match tag {
                    Expression::Symbol(sym) if sym == "else" => block.clone(),
                    _ => {
                        return Err(Error::Invalid(
                            "consider providing else case for switch statement".to_string(),
                        ))
                    }
                }
            }
            _ => {
                return Err(Error::Invalid(
                    "invalid syntax for switch statement".to_string(),
                ))
            }
        };

        for case in rest {
            if let Expression::List(case_list) = case {
                let [cond, block] = &case_list[..] else {
                    return Err(Error::Invalid(
                        "each case of switch statement must followed by 1 block expression".to_string(),
                    ));
                };

                result = Expression::List(vec![
                    Expression::Symbol("if".to_string()),
                    cond.clone(),
                    block.clone(),
                    result.clone(),
                ])
            } else {
                return Err(Error::Invalid("invalid syntax for switch".to_string()));
            };
        }
        Ok(result)
    } else {
        Err(Error::Invalid("invalid syntax for switch".to_string()))
    }
}

pub fn transform_for_to_while(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, init, cond, modifier, body] = list else {
        return Err(Error::Invalid("invalid syntax for for-loop".to_string()));
    };

    dbg!(init, cond, modifier, body);

    Ok(Expression::List(vec![
        Expression::Symbol("begin".to_string()),
        init.clone(),
        Expression::List(vec![
            Expression::Symbol("while".to_string()),
            cond.clone(),
            Expression::List(vec![
                Expression::Symbol("begin".to_string()),
                body.clone(),
                modifier.clone(),
            ]),
        ]),
    ]))
}

pub fn transform_increament(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, var] = list else {
        return Err(Error::Invalid("invalid syntax for ++".to_string()));
    };

    Ok(transform_shorthand_assignment(
        Expression::Symbol("+".to_string()),
        var.clone(),
        Expression::Number(1.0),
    ))
}

pub fn transform_decreament(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, var] = list else {
        return Err(Error::Invalid("invalid syntax for --".to_string()));
    };

    Ok(transform_shorthand_assignment(
        Expression::Symbol("-".to_string()),
        var.clone(),
        Expression::Number(1.0),
    ))
}

pub fn transform_increament_assign(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, left, right] = list else {
        return Err(Error::Invalid("invalid syntax for +=".to_string()));
    };

    Ok(transform_shorthand_assignment(
        Expression::Symbol("+".to_string()),
        left.clone(),
        right.clone(),
    ))
}

pub fn transform_decreament_assign(list: &[Expression]) -> Result<Expression, Error> {
    let [_tag, left, right] = list else {
        return Err(Error::Invalid("invalid syntax for -=".to_string()));
    };

    Ok(transform_shorthand_assignment(
        Expression::Symbol("-".to_string()),
        left.clone(),
        right.clone(),
    ))
}

fn transform_shorthand_assignment(
    op: Expression,
    left: Expression,
    right: Expression,
) -> Expression {
    Expression::List(vec![
        Expression::Symbol("set".to_string()),
        left.clone(),
        Expression::List(vec![op, left, right]),
    ])
}
