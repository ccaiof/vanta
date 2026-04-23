use vanta_ast::{ImportDecl, PackDecl};
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
        let body = self.parse_body()?;
        self.expect(TokenKind::RBrace)?;

        Ok(vanta_ast::FunctionDecl {
            visibility,
            name,
            params,
            return_type,
            body,
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

    fn parse_body(&mut self) -> Result<Vec<vanta_ast::Expr>, Diagnostic> {
        let mut exprs = Vec::new();

        while !self.check(&TokenKind::RBrace) {
            exprs.push(self.parse_expression()?);
        }

        Ok(exprs)
    }

    fn parse_expression(&mut self) -> Result<vanta_ast::Expr, Diagnostic> {
        if self.check(&TokenKind::Return) {
            self.advance();

            if self.check(&TokenKind::RBrace) {
                return Ok(vanta_ast::Expr::Return(vanta_ast::ReturnExpr {
                    value: None,
                }));
            }

            let value = self.parse_expression()?;

            return Ok(vanta_ast::Expr::Return(vanta_ast::ReturnExpr {
                value: Some(Box::new(value)),
            }));
        }

        let mut expr = self.parse_primary()?;

        while self.check(&TokenKind::Dot) {
            self.advance();
            let property = self.expect_identifier()?;

            expr = vanta_ast::Expr::PropertyAccess(vanta_ast::PropertyAccess {
                object: Box::new(expr),
                property,
            });
        }

        if self.check(&TokenKind::Equal) {
            self.advance();
            let value = self.parse_expression()?;

            expr = vanta_ast::Expr::Assignment(vanta_ast::Assignment {
                target: Box::new(expr),
                value: Box::new(value),
            });
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<vanta_ast::Expr, Diagnostic> {
        let token = self.peek().ok_or(Diagnostic::UnexpectedEof)?;

        match &token.kind {
            TokenKind::StringLiteral(value) => {
                let value = value.clone();
                self.advance();

                Ok(vanta_ast::Expr::StringLiteral(vanta_ast::StringLiteral {
                    value,
                }))
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.check(&TokenKind::LParen) {
                    self.advance();
                    let args = self.parse_arguments()?;
                    self.expect(TokenKind::RParen)?;

                    return Ok(vanta_ast::Expr::Call(vanta_ast::Call {
                        callee: name,
                        args,
                    }));
                }

                Ok(vanta_ast::Expr::Identifier(vanta_ast::Identifier { name }))
            }
            other => Err(Diagnostic::UnexpectedToken {
                expected: "expression".to_string(),
                found: format!("{other:?}"),
            }),
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<vanta_ast::Expr>, Diagnostic> {
        let mut args = Vec::new();

        while !self.check(&TokenKind::RParen) {
            args.push(self.parse_expression()?);

            if self.check(&TokenKind::Comma) {
                self.advance();

                if self.check(&TokenKind::RParen) {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(args)
    }

    pub fn parse_pack(&mut self) -> Result<PackDecl, Diagnostic> {
        self.expect(TokenKind::Pack)?;

        let name = self.parse_pack_name()?;

        Ok(PackDecl { name })
    }

    pub fn parse_import(&mut self) -> Result<ImportDecl, Diagnostic> {
        self.expect(TokenKind::Import)?;

        let name = self.parse_pack_name()?;

        Ok(ImportDecl { name })
    }

    pub fn parse_pack_name(&mut self) -> Result<String, Diagnostic> {
        let mut name = self.expect_identifier()?;

        while self.check(&TokenKind::Dot) {
            self.advance(); // consome o '.'

            let next = self.expect_identifier()?;
            name.push('.');
            name.push_str(&next);
        }

        Ok(name)
    }
}
