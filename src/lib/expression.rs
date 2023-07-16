#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expression>),
}
