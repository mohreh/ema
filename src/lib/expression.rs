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
    Object(Object), // oop and modules
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub idx: usize,
    pub parent: Option<Rc<RefCell<Object>>>,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Expression::Void => "nil".to_string(),
            Expression::Boolean(bool) => bool.to_string(),
            Expression::Number(num) => num.to_string(),
            Expression::String(s) => s.clone(),
            Expression::Symbol(k) => k.clone(),
            Expression::List(list) => {
                let mut str = "(".to_string();
                str += &list
                    .iter()
                    .map(|exp| exp.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                str += ")";
                str.to_string()
            }
            Expression::Function(params, _, _) => format!("fn({})", params.join(", ")).to_string(),
            Expression::Object(..) => "class ".to_string(),
        };
        write!(f, "{}", str)
    }
}
