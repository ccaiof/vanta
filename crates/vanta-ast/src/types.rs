#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Int,
    Void,
    Custom(String),
}