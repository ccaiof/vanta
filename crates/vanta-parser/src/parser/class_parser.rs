use vanta_ast::ClassDecl;
use vanta_diagnostics::Diagnostic;
use vanta_lexer::TokenKind;

use crate::parser::Parser;

impl Parser {
    pub fn parse_class(&mut self) -> Result<ClassDecl, Diagnostic> {
        self.expect(TokenKind::Class)?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LParen)?;
        let fields = self.parse_fields()?;
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::LBrace)?;
        let methods = self.parse_methods()?;
        self.expect(TokenKind::RBrace)?;

        Ok(ClassDecl {
            name,
            fields,
            methods,
        })
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
}
