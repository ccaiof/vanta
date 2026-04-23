use std::collections::HashSet;

use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;

pub fn check_imports(program: &Program) -> Result<(), Diagnostic> {
    let mut seen = HashSet::new();

    for import in &program.imports {
        if import.name.trim().is_empty() {
            return Err(Diagnostic::InvalidSyntax {
                message: "import name cannot be empty".to_string(),
            });
        }

        if !seen.insert(import.name.as_str()) {
            return Err(Diagnostic::InvalidSyntax {
                message: format!("duplicate import '{}'", import.name),
            });
        }
    }

    Ok(())
}