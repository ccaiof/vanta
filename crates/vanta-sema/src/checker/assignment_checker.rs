use vanta_ast::{Expr, Mutability};

use vanta_diagnostics::Diagnostic;

use crate::{TypeContext, infer_expr_type};

pub fn check_assignments(program: &vanta_ast::Program) -> Result<(), Diagnostic> {
    for class in &program.classes {
        for method in &class.methods {
            let context = TypeContext {
                class_name: &class.name,
                fields: &class.fields,
                params: &method.params,
            };

            for expr in &method.body {
                check_expr_assignment(expr, &context, class.name.as_str(), method.name.as_str())?;
            }
        }
    }

    Ok(())
}

fn check_expr_assignment(
    expr: &Expr,
    context: &TypeContext,
    class_name: &str,
    method_name: &str,
) -> Result<(), Diagnostic> {
    if let Expr::Assignment(assignment) = expr {
        let property_access = match assignment.target.as_ref() {
            Expr::PropertyAccess(property_access) => property_access,
            _ => {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!(
                        "assignment in '{}.{}' must target a property",
                        class_name, method_name
                    ),
                });
            }
        };

        match property_access.object.as_ref() {
            Expr::Identifier(identifier) if identifier.name == "self" => {}
            _ => {
                return Err(Diagnostic::InvalidSyntax {
                    message: format!(
                        "assignment in '{}.{}' currently only supports self.property",
                        class_name, method_name
                    ),
                });
            }
        }

        let field = context
            .fields
            .iter()
            .find(|field| field.name == property_access.property)
            .ok_or_else(|| Diagnostic::InvalidSyntax {
                message: format!(
                    "property '{}' was not found in class '{}'",
                    property_access.property, context.class_name
                ),
            })?;

        if field.mutability != Mutability::Mut {
            return Err(Diagnostic::InvalidSyntax {
                message: format!(
                    "property '{}.{}' is immutable and cannot receive assignment",
                    context.class_name, field.name
                ),
            });
        }

        let value_type = infer_expr_type(assignment.value.as_ref(), context)?;

        if value_type != field.ty {
            return Err(Diagnostic::InvalidSyntax {
                message: format!(
                    "assignment to '{}.{}' expects {:?} but found {:?}",
                    context.class_name, field.name, field.ty, value_type
                ),
            });
        }
    }

    Ok(())
}
