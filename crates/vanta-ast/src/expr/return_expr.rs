use crate::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpr {
    pub value: Option<Box<Expr>>,
}