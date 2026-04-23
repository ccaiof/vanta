use vanta_ast::{Expr, Param, Type};
use vanta_diagnostics::Diagnostic;

pub struct TypeContext<'a> {
    pub params: &'a [Param],
}

pub fn infer_expr_type(expr: &Expr, context: &TypeContext) -> Result<Type, Diagnostic> {
    match expr {
        Expr::StringLiteral(_) => Ok(Type::String),

        Expr::Identifier(identifier) => {
            let param = context
                .params
                .iter()
                .find(|param| param.name == identifier.name)
                .ok_or_else(|| Diagnostic::InvalidSyntax {
                    message: format!(
                        "type inference for identifier '{}' is not implemented yet",
                        identifier.name
                    ),
                })?;

            Ok(param.ty.clone())
        }

        Expr::PropertyAccess(_) => Err(Diagnostic::InvalidSyntax {
            message: "type inference for property access is not implemented yet".to_string(),
        }),

        Expr::Assignment(_) => Err(Diagnostic::InvalidSyntax {
            message: "type inference for assignment is not implemented yet".to_string(),
        }),

        Expr::Call(call) => Err(Diagnostic::InvalidSyntax {
            message: format!(
                "type inference for call '{}' is not implemented yet",
                call.callee
            ),
        }),

        Expr::Return(_) => Err(Diagnostic::InvalidSyntax {
            message: "cannot infer type directly from return expression".to_string(),
        }),
    }
}
