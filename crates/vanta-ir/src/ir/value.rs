#[derive(Debug, Clone, PartialEq)]
pub enum IrValue {
    StringLiteral(String),
    Identifier(String),
}
