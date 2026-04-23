use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct IrFunction {
    pub name: String,
    pub instructions: Vec<Instruction>,
}
