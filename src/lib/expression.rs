use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Void,
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
    Object(usize, Option<usize>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        let str = match self {
            Void => "nil".to_string(),
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
            Function(params, _, _) => format!("fn({})", params.join(", ")).to_string(),
            Object(..) => "class ".to_string(),
        };
        write!(f, "{}", str)
    }
}
