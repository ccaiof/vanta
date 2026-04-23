use vanta_diagnostics::Diagnostic;
use vanta_lexer::TokenKind;

use crate::parser::Parser;

impl Parser {
    pub fn parse_methods(&mut self) -> Result<Vec<vanta_ast::FunctionDecl>, Diagnostic> {
        let mut methods = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            methods.push(self.parse_method()?);
        }

        Ok(methods)
    }

    fn parse_method(&mut self) -> Result<vanta_ast::FunctionDecl, Diagnostic> {
        let visibility = self.parse_visibility()?;
        self.expect(TokenKind::Function)?;
        let name = self.expect_identifier()?;

        self.expect(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;

        let return_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenKind::LBrace)?;
        self.expect(TokenKind::RBrace)?;

        Ok(vanta_ast::FunctionDecl {
            visibility,
            name,
            params,
            return_type,
            body: vec![],
        })
    }

    fn parse_params(&mut self) -> Result<Vec<vanta_ast::Param>, Diagnostic> {
        let mut params = Vec::new();

        while !self.check(&TokenKind::RParen) {
            let name = self.expect_identifier()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;

            params.push(vanta_ast::Param { name, ty });

            if self.check(&TokenKind::Comma) {
                self.advance();

                if self.check(&TokenKind::RParen) {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(params)
    }
}
