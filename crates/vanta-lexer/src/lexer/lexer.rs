use vanta_diagnostics::Diagnostic;

use crate::{Token, TokenKind};

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            input: source.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Diagnostic> {
        let mut tokens = Vec::new();

        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\n' | '\r' | '\t' => {
                    self.input.next();
                }
                '(' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::LParen,
                    });
                }
                ')' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::RParen,
                    });
                }
                '{' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::LBrace,
                    });
                }
                '}' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::RBrace,
                    });
                }
                ':' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::Colon,
                    });
                }
                ',' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                    });
                }
                '.' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::Dot,
                    });
                }
                '=' => {
                    self.input.next();
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                    });
                }
                '"' => {
                    tokens.push(self.read_string()?);
                }
                c if is_identifier_start(c) => {
                    tokens.push(self.read_identifier_or_keyword());
                }
                _ => {
                    return Err(Diagnostic::InvalidSyntax {
                        message: format!("unexpected character: {ch}"),
                    });
                }
            }
        }

        tokens.push(Token {
            kind: TokenKind::Eof,
        });

        Ok(tokens)
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut value = String::new();

        while let Some(&ch) = self.input.peek() {
            if is_identifier_part(ch) {
                value.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        let kind = match value.as_str() {
            "class" => TokenKind::Class,
            "pack" => TokenKind::Pack,
            "import" => TokenKind::Import,
            "pub" => TokenKind::Pub,
            "priv" => TokenKind::Priv,
            "val" => TokenKind::Val,
            "mut" => TokenKind::Mut,
            "function" => TokenKind::Function,
            "set" => TokenKind::Set,
            "String" => TokenKind::StringType,
            "Int" => TokenKind::IntType,
            "Void" => TokenKind::VoidType,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier(value),
        };

        Token { kind }
    }

    fn read_string(&mut self) -> Result<Token, Diagnostic> {
        self.input.next();

        let mut value = String::new();

        while let Some(ch) = self.input.next() {
            if ch == '"' {
                return Ok(Token {
                    kind: TokenKind::StringLiteral(value),
                });
            }

            value.push(ch);
        }

        Err(Diagnostic::InvalidSyntax {
            message: "unterminated string literal".to_string(),
        })
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_identifier_part(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

pub fn lex(source: &str) -> Result<Vec<Token>, Diagnostic> {
    Lexer::new(source).lex()
}
