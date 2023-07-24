use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expression>),
    Function(
        Vec<String>,
        Rc<RefCell<Expression>>,
        usize, // env
    ),
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
            Function(_, _, _) => "Function ()".to_string(),
        };
        write!(f, "{}", str)
    }
}
