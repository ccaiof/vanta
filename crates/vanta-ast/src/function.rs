use crate::{Type, Visibility};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub visibility: Visibility,
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}