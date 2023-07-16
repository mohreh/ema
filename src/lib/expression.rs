use crate::literal::Literal;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Literal(Literal),
    List(Vec<Expression>),
}
