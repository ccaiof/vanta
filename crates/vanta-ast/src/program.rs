use crate::{ClassDecl, ImportDecl, PackDecl};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub pack: PackDecl,
    pub imports: Vec<ImportDecl>,
    pub classes: Vec<ClassDecl>,
}
