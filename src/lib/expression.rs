use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let str = match self {
            Boolean(bool) => bool.to_string(),
            Number(num) => num.to_string(),
            String(s) => s.clone(),
            Symbol(k) => k.clone(),
            List(list) => {
                let mut str = "(".to_string();
                for exp in list {
                    str += format!("{}", exp).as_str();
                }
                str += ")";

                str.to_string()
            }
        };
        write!(f, "{}", str)
    }
}
