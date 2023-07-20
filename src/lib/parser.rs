use crate::{
    error::Error,
    expression::Expression,
    lexer::{tokenize, Token},
};

pub fn parser(program: &str) -> Result<Expression, Error> {
    let res = tokenize(program);
    match res {
        Ok(mut tokens) => {
            tokens.reverse();
            parse(&mut tokens)
        }
        Err(err) => Err(err),
    }
}

fn parse(tokens: &mut Vec<Token>) -> Result<Expression, Error> {
    let mut res: Vec<Expression> = Vec::new();

    while !tokens.is_empty() {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => res.push(Expression::Number(num)),
                Token::String(s) => res.push(Expression::String(s)),
                Token::Symbol(k) => res.push(Expression::Symbol(k)),
                Token::LParen => res.push(parse(tokens)?),
                Token::RParen => {
                    return Ok(Expression::List(res));
                }
            }
        }
    }

    Ok(Expression::List(res))
}
