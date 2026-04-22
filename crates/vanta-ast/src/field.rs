use crate::{Type, Visibility, Mutability};

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub name: String,
    pub ty: Type,
}