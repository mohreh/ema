#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    Invalid(String),
    Reason(String),
    Reference(String),
    Token(String),
    Parse(String),
}
