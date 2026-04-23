use vanta_ast::{Expr, Program, Type};
use vanta_diagnostics::Diagnostic;

pub fn check_return_types(program: &Program) -> Result<(), Diagnostic> {
    for class in &program.classes {
        for method in &class.methods {
            let Some(return_type) = method.return_type.as_ref() else {
                continue;
            };

            for expr in &method.body {
                if let Expr::Return(ret) = expr {
                    match return_type {
                        Type::Void => {
                            if ret.value.is_some() {
                                return Err(Diagnostic::InvalidSyntax {
                                    message: format!(
                                        "method '{}.{}' returns Void and cannot return a value",
                                        class.name, method.name
                                    ),
                                });
                            }
                        }
                        Type::String => {
                            let Some(value) = ret.value.as_ref() else {
                                return Err(Diagnostic::InvalidSyntax {
                                    message: format!(
                                        "method '{}.{}' returns String and must return a String value",
                                        class.name, method.name
                                    ),
                                });
                            };

                            if !matches!(value.as_ref(), Expr::StringLiteral(_)) {
                                return Err(Diagnostic::InvalidSyntax {
                                    message: format!(
                                        "method '{}.{}' returns String and currently only accepts string literals in return",
                                        class.name, method.name
                                    ),
                                });
                            }
                        }
                        Type::Int => {
                            return Err(Diagnostic::InvalidSyntax {
                                message: format!(
                                    "return type checking for Int is not implemented yet in method '{}.{}'",
                                    class.name, method.name
                                ),
                            });
                        }
                        Type::Custom(type_name) => {
                            return Err(Diagnostic::InvalidSyntax {
                                message: format!(
                                    "return type checking for custom type '{}' is not implemented yet in method '{}.{}'",
                                    type_name, class.name, method.name
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
