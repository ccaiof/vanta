use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
}
