use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().collect::<Vec<char>>();

    if chars.is_empty() {
        return Ok(tokens);
    }

    while !chars.is_empty() {
        let mut ch = chars.remove(0);
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '"' => {
                let mut word = String::new();
                while !chars.is_empty() && chars[0] != '"' {
                    word.push(chars.remove(0));
                }

                if !chars.is_empty() && chars[0] == '"' {
                    chars.remove(0);
                } else {
                    return Err(Error::Token("suspended string definition".to_string()));
                }

                tokens.push(Token::String(word));
            }
            ';' => {
                while !chars.is_empty() && chars[0] != '\n' {
                    chars.remove(0);
                }
            }
            _ => {
                let mut word = String::new();

                word.push(ch);

                while !chars.is_empty() && !ch.is_whitespace() {
                    match chars[0] {
                        ')' | '(' | ';' => break,
                        _ => ch = chars.remove(0),
                    }

                    word.push(ch);
                }

                word = word.trim().to_string();

                if !word.is_empty() {
                    if let Ok(num) = word.parse::<f64>() {
                        tokens.push(Token::Number(num))
                    } else {
                        tokens.push(Token::Symbol(word))
                    }
                }
            }
        }
    }

    Ok(tokens)
}
