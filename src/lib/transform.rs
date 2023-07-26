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
