use vanta_ast::{Program, Type, Visibility};
use vanta_diagnostics::Diagnostic;

pub fn check_entrypoint(program: &Program) -> Result<(), Diagnostic> {
    let app_classes: Vec<_> = program
        .classes
        .iter()
        .filter(|class| class.name == "App")
        .collect();

    if app_classes.is_empty() {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint class 'App' was not found".to_string(),
        });
    }

    if app_classes.len() > 1 {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint class 'App' must be unique".to_string(),
        });
    }

    let app_class = app_classes[0];

    let main_methods: Vec<_> = app_class
        .methods
        .iter()
        .filter(|method| method.name == "main")
        .collect();

    if main_methods.is_empty() {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' was not found".to_string(),
        });
    }

    if main_methods.len() > 1 {
        return Err(Diagnostic::InvalidSyntax {
            message: "entrypoint method 'App.main' must be unique".to_string(),
        });
    }

    let main_method = main_methods[0];

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