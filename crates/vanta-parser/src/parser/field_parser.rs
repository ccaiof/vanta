use vanta_diagnostics::Diagnostic;
use vanta_lexer::TokenKind;

use crate::parser::Parser;

impl Parser {
    pub fn parse_visibility(&mut self) -> Result<vanta_ast::Visibility, Diagnostic> {
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

    pub fn parse_mutability(&mut self) -> Result<vanta_ast::Mutability, Diagnostic> {
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

    pub fn parse_type(&mut self) -> Result<vanta_ast::Type, Diagnostic> {
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
}
