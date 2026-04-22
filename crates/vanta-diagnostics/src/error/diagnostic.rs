use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Diagnostic {
    #[error("unexpected token: expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: String },

    #[error("unexpected end of input")]
    UnexpectedEof,

    #[error("invalid syntax: {message}")]
    InvalidSyntax { message: String },
}
