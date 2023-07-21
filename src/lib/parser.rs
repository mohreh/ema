use std::collections::VecDeque;

use crate::{
    error::Error,
    expression::Expression,
    lexer::{tokenize, Token},
};

pub fn parse(program: &str) -> Result<Expression, Error> {
    let res = tokenize(program);
    match res {
        Ok(mut tokens) => {
            tokens.reverse();
            parse_tokens(&mut tokens, &mut VecDeque::new())
        }
        Err(err) => Err(err),
    }
}

fn parse_tokens(
    tokens: &mut Vec<Token>,
    paren_stack: &mut VecDeque<()>,
) -> Result<Expression, Error> {
    let mut res: Vec<Expression> = Vec::new();

    while !tokens.is_empty() {
        if let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => res.push(Expression::Number(num)),
                Token::String(s) => res.push(Expression::String(s)),
                Token::Symbol(k) => res.push(Expression::Symbol(k)),
                Token::LParen => {
                    paren_stack.push_back(());
                    res.push(parse_tokens(tokens, paren_stack)?)
                }
                Token::RParen => {
                    if paren_stack.pop_back().is_none() {
                        return Err(Error::Parse("unbalanced parens".to_string()));
                    }
                    return Ok(Expression::List(res));
                }
            }
        }
    }

    if !paren_stack.is_empty() {
        return Err(Error::Parse("unbalanced parens".to_string()));
    }

    Ok(Expression::List(res))
}
