use std::fmt::Display;

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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let str = match self {
            Invalid(err) => format!("invalid statement: {}", err),
            Parse(err) => format!("parsing error: {}", err),
            Reason(err) => err.to_string(),
            Reference(err) => format!("reference error: {}", err),
            Token(err) => format!("missing token: {}", err),
        };
        write!(f, "{}", str)
    }
}
