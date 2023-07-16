#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
}
