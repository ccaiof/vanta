use vanta_ast::{Expr, FieldDecl, Param, Type};
use vanta_diagnostics::Diagnostic;

use crate::infer_builtin_call_type;

pub struct TypeContext<'a> {
    pub class_name: &'a str,
    pub fields: &'a [FieldDecl],
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

        Expr::PropertyAccess(property_access) => match property_access.object.as_ref() {
            Expr::Identifier(identifier) if identifier.name == "self" => {
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

                Ok(field.ty.clone())
            }
            _ => Err(Diagnostic::InvalidSyntax {
                message: "type inference for property access is only implemented for self.property"
                    .to_string(),
            }),
        },

        Expr::Assignment(_) => Err(Diagnostic::InvalidSyntax {
            message: "type inference for assignment is not implemented yet".to_string(),
        }),

        Expr::Call(call) => infer_builtin_call_type(call),

        Expr::Return(_) => Err(Diagnostic::InvalidSyntax {
            message: "cannot infer type directly from return expression".to_string(),
        }),
    }
}
