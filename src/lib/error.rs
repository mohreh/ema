#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    Invalid(String),
    Reason(String),
    Reference(String),
    Token(String),
    Parse(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Reason(value.to_string())
    }
}
