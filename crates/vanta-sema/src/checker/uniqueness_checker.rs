use std::collections::HashSet;

use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;

pub fn check_uniqueness(program: &Program) -> Result<(), Diagnostic> {
    let mut class_names = HashSet::new();

    for class in &program.classes {
        if !class_names.insert(&class.name) {
            return Err(Diagnostic::InvalidSyntax {
                message: format!("class '{}' is already defined", class.name),
            });
        }

        let mut method_names = HashSet::new();

        for method in &class.methods {
            if !method_names.insert(&method.name) {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!("method '{}.{}' is already defined", class.name, method.name),
                });
            }
        }
    }

    Ok(())
}
