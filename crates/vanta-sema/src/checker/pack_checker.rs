use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;

pub fn check_pack(program: &Program) -> Result<(), Diagnostic> {
    if program.pack.name.trim().is_empty() {
        return Err(Diagnostic::InvalidSyntax {
            message: "pack name cannot be empty".to_string(),
        });
    }

    Ok(())
}
