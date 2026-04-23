use vanta_diagnostics::Diagnostic;
use vanta_lexer::{Token, TokenKind};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn check(&self, expected: &TokenKind) -> bool {
        match self.peek() {
            Some(token) => &token.kind == expected,
            None => false,
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> Result<(), Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        if token.kind == expected {
            self.advance();
            Ok(())
        } else {
            Err(Diagnostic::UnexpectedToken {
                expected: format!("{expected:?}"),
                found: format!("{:?}", token.kind),
            })
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::Eof
            })
        )
    }

    pub fn expect_identifier(&mut self) -> Result<String, Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        match &token.kind {
            TokenKind::Identifier(value) => {
                let value = value.clone();
                self.advance();
                Ok(value)
            }
            other => Err(Diagnostic::UnexpectedToken {
                expected: "Identifier".to_string(),
                found: format!("{other:?}"),
            }),
        }
    }
}
