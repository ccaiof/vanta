#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Class,
    Pack,
    Import,
    Pub,
    Priv,
    Val,
    Mut,
    Function,
    Set,

    StringType,
    IntType,
    VoidType,

    Identifier(String),
    StringLiteral(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Dot,
    Equal,

    Eof,

    Return,
}
