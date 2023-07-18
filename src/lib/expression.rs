#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Boolean(bool),
    Number(f64),
    String(String),
    // Symbol(String), I don't know what to do yet. go on with strings or check it later
    List(Vec<Expression>),
}
