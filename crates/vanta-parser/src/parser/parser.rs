use vanta_ast::{ClassDecl, Program};
use vanta_diagnostics::Diagnostic;
use vanta_lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, Diagnostic> {
        let mut classes = Vec::new();

        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }

        Ok(Program { classes })
    }

    fn parse_class(&mut self) -> Result<ClassDecl, Diagnostic> {
        self.expect(TokenKind::Class)?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        self.expect(TokenKind::RBrace)?;

        Ok(ClassDecl {
            name,
            fields: vec![],
            methods: vec![],
        })
    }

    fn expect(&mut self, expected: TokenKind) -> Result<(), Diagnostic> {
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

    fn expect_identifier(&mut self) -> Result<String, Diagnostic> {
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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token {
                kind: TokenKind::Eof
            })
        )
    }
}
