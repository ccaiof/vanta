use vanta_ast::{Call, Type};
use vanta_diagnostics::Diagnostic;

pub fn infer_builtin_call_type(call: &Call) -> Result<Type, Diagnostic> {
    match call.callee.as_str() {
        "print" => Ok(Type::Void),
        _ => Err(Diagnostic::InvalidSyntax {
            message: format!(
                "type inference for call '{}' is not implemented yet",
                call.callee
            ),
        }),
    }
}
