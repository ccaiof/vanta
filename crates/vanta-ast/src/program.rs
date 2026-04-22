use crate::ClassDecl;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub classes: Vec<ClassDecl>,
}