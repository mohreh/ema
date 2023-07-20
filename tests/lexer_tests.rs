use ema::lexer::{tokenize, Token};

use Token::*;
#[test]
fn return_empty_vec_if_input_is_empty() {
    let res = tokenize("");
    assert!(res.is_ok());
    assert!(res.unwrap().is_empty());
}

#[test]
fn tokenize_correctly() {
    let res = tokenize(
        "
(
    (var x 10)
    (var y 10)

    (set x 12)
    (if (> x 10)
        (+ y (* x x))
        (* x (+ y y))
    )
)
",
    );
    assert_eq!(
        res,
        Ok(vec![
            LParen,
            // define x
            LParen,
            Symbol("var".to_string()),
            Symbol("x".to_string()),
            Number(10.0),
            RParen,
            // define y
            LParen,
            Symbol("var".to_string()),
            Symbol("y".to_string()),
            Number(10.0),
            RParen,
            // assign x
            LParen,
            Symbol("set".to_string()),
            Symbol("x".to_string()),
            Number(12.0),
            RParen,
            //
            // if statement
            LParen,
            Symbol("if".to_string()),
            //condition
            LParen,
            Symbol(">".to_string()),
            Symbol("x".to_string()),
            Number(10.0),
            RParen,
            // consequent
            LParen,
            Symbol("+".to_string()),
            Symbol("y".to_string()),
            LParen,
            Symbol("*".to_string()),
            Symbol("x".to_string()),
            Symbol("x".to_string()),
            RParen,
            RParen,
            // alternate
            LParen,
            Symbol("*".to_string()),
            Symbol("x".to_string()),
            LParen,
            Symbol("+".to_string()),
            Symbol("y".to_string()),
            Symbol("y".to_string()),
            RParen,
            RParen,
            RParen,
            RParen
        ])
    )
}
