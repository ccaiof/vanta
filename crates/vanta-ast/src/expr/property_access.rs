use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyAccess {
    pub object: Box<Expr>,
    pub property: String,
}
