use vanta_ast::{Program, Type, Visibility};
use vanta_diagnostics::Diagnostic;

pub fn check_entrypoint(program: &Program) -> Result<(), Diagnostic> {
    let app_class = program
        .classes
        .iter()
        .find(|class| class.name == "App")
        .ok_or_else(|| Diagnostic::InvalidSyntax {
            message: "entrypoint class 'App' was not found".to_string(),
        })?;

    let main_method = app_class
        .methods
        .iter()
        .find(|method| method.name == "main")
        .ok_or_else(|| Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' was not found".to_string(),
        })?;

    if main_method.visibility != Visibility::Pub {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' must be public".to_string(),
        });
    }

    if !main_method.params.is_empty() {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' must not have parameters".to_string(),
        });
    }

    if main_method.return_type != Some(Type::Void) {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' must return Void".to_string(),
        });
    }

    Ok(())
}
