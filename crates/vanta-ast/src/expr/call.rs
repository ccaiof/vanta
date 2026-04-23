use crate::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: String,
    pub args: Vec<Expr>,
}
