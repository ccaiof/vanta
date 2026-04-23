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
        let fields = self.parse_fields()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        self.expect(TokenKind::RBrace)?;

        Ok(ClassDecl {
            name,
            fields,
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

    fn parse_fields(&mut self) -> Result<Vec<vanta_ast::FieldDecl>, Diagnostic> {
        let mut fields = Vec::new();

        while !self.check(&TokenKind::RParen) {
            fields.push(self.parse_field()?);

            if self.check(&TokenKind::Comma) {
                self.advance();

                if self.check(&TokenKind::RParen) {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<vanta_ast::FieldDecl, Diagnostic> {
        let visibility = self.parse_visibility()?;
        let mutability = self.parse_mutability()?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;

        Ok(vanta_ast::FieldDecl {
            visibility,
            mutability,
            name,
            ty,
        })
    }

    fn parse_visibility(&mut self) -> Result<vanta_ast::Visibility, Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        match &token.kind {
            TokenKind::Pub => {
                self.advance();
                Ok(vanta_ast::Visibility::Pub)
            }
            TokenKind::Priv => {
                self.advance();
                Ok(vanta_ast::Visibility::Priv)
            }
            other => Err(Diagnostic::UnexpectedToken {
                expected: "Pub or Priv".to_string(),
                found: format!("{other:?}"),
            }),
        }
    }

    fn parse_mutability(&mut self) -> Result<vanta_ast::Mutability, Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        match &token.kind {
            TokenKind::Val => {
                self.advance();
                Ok(vanta_ast::Mutability::Val)
            }
            TokenKind::Mut => {
                self.advance();
                Ok(vanta_ast::Mutability::Mut)
            }
            other => Err(Diagnostic::UnexpectedToken {
                expected: "Val or Mut".to_string(),
                found: format!("{other:?}"),
            }),
        }
    }

    fn parse_type(&mut self) -> Result<vanta_ast::Type, Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        match &token.kind {
            TokenKind::StringType => {
                self.advance();
                Ok(vanta_ast::Type::String)
            }
            TokenKind::IntType => {
                self.advance();
                Ok(vanta_ast::Type::Int)
            }
            TokenKind::VoidType => {
                self.advance();
                Ok(vanta_ast::Type::Void)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(vanta_ast::Type::Custom(name))
            }
            other => Err(Diagnostic::UnexpectedToken {
                expected: "type".to_string(),
                found: format!("{other:?}"),
            }),
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        match self.peek() {
            Some(token) => &token.kind == expected,
            None => false,
        }
    }
}
