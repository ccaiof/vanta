use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;

use crate::parser::Parser;

impl Parser {
    pub fn parse_program(&mut self) -> Result<Program, Diagnostic> {
        let mut classes = Vec::new();

        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }

        Ok(Program { classes })
    }
}
