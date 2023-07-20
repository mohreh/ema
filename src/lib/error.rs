#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    Reason(String),
    Reference(String),
    Token(String),
    Parse(String),
}
