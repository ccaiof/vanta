use vanta_ast::{Expr, Program, Type};

use vanta_diagnostics::Diagnostic;

pub fn check_returns(program: &Program) -> Result<(), Diagnostic> {
    for class in &program.classes {
        for method in &class.methods {
            match method.return_type.as_ref() {
                Some(Type::Void) => check_void_method_returns(
                    class.name.as_str(),
                    method.name.as_str(),
                    &method.body,
                )?,
                Some(Type::String) => check_string_method_returns(
                    class.name.as_str(),
                    method.name.as_str(),
                    &method.body,
                )?,
                Some(Type::Int) => check_non_void_method_has_return(
                    class.name.as_str(),
                    method.name.as_str(),
                    "Int",
                    &method.body,
                )?,
                Some(Type::Custom(type_name)) => check_non_void_method_has_return(
                    class.name.as_str(),
                    method.name.as_str(),
                    type_name,
                    &method.body,
                )?,
                None => {}
            }
        }
    }

    Ok(())
}

fn check_void_method_returns(
    class_name: &str,
    method_name: &str,
    body: &[Expr],
) -> Result<(), Diagnostic> {
    for expr in body {
        if let Expr::Return(ret) = expr {
            if ret.value.is_some() {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!(
                        "method '{}.{}' returns Void and cannot return a value",
                        class_name, method_name
                    ),
                });
            }
        }
    }

    Ok(())
}

fn check_string_method_returns(
    class_name: &str,
    method_name: &str,
    body: &[Expr],
) -> Result<(), Diagnostic> {
    let last = body.last().ok_or_else(|| Diagnostic::InvalidSyntax {
        message: format!(
            "method '{}.{}' returns String and must end with return",
            class_name, method_name
        ),
    })?;

    match last {
        Expr::Return(ret) => {
            if ret.value.is_none() {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!(
                        "method '{}.{}' returns String and must return a value",
                        class_name, method_name
                    ),
                });
            }
            Ok(())
        }
        _ => Err(Diagnostic::InvalidSyntax {
            message: format!(
                "method '{}.{}' returns String and must end with return",
                class_name, method_name
            ),
        }),
    }
}

fn check_non_void_method_has_return(
    class_name: &str,
    method_name: &str,
    return_type: &str,
    body: &[Expr],
) -> Result<(), Diagnostic> {
    let last = body.last().ok_or_else(|| Diagnostic::InvalidSyntax {
        message: format!(
            "method '{}.{}' returns {} and must end with return",
            class_name, method_name, return_type
        ),
    })?;

    match last {
        Expr::Return(ret) => {
            if ret.value.is_none() {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!(
                        "method '{}.{}' returns {} and must return a value",
                        class_name, method_name, return_type
                    ),
                });
            }
            Ok(())
        }
        _ => Err(Diagnostic::InvalidSyntax {
            message: format!(
                "method '{}.{}' returns {} and must end with return",
                class_name, method_name, return_type
            ),
        }),
    }
}
