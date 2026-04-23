use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;
use vanta_lexer::TokenKind;

use crate::parser::Parser;

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, Diagnostic> {
        let pack = self.parse_pack()?;

        let mut imports = Vec::new();

        while self.check(&TokenKind::Import) {
            imports.push(self.parse_import()?);
        }

        let mut classes = Vec::new();

        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }

        Ok(Program {
            pack,
            imports,
            classes,
        })
    }
}
