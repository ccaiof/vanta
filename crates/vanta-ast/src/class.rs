use crate::{FieldDecl, FunctionDecl};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<FunctionDecl>,
}
