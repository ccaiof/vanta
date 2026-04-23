use vanta_ast::{Expr, Program, Type};
use vanta_diagnostics::Diagnostic;

use crate::infer_expr_type;

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
                        expected_type => {
                            let Some(value) = ret.value.as_ref() else {
                                return Err(Diagnostic::InvalidSyntax {
                                    message: format!(
                                        "method '{}.{}' returns {:?} and must return a value",
                                        class.name, method.name, expected_type
                                    ),
                                });
                            };

                            let inferred_type = infer_expr_type(value)?;

                            if &inferred_type != expected_type {
                                return Err(Diagnostic::InvalidSyntax {
                                    message: format!(
                                        "method '{}.{}' returns {:?} but found {:?}",
                                        class.name, method.name, expected_type, inferred_type
                                    ),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
