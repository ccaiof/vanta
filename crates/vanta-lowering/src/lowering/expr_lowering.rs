use vanta_ast::Expr;
use vanta_diagnostics::Diagnostic;
use vanta_ir::{Instruction, IrValue};

use crate::lowering::lowerer::Lowerer;

impl Lowerer {
    pub(crate) fn lower_expr(
        &mut self,
        expr: &Expr,
        instructions: &mut Vec<Instruction>,
    ) -> Result<Option<IrValue>, Diagnostic> {
        match expr {
            Expr::StringLiteral(value) => Ok(Some(IrValue::StringLiteral(value.value.clone()))),

            Expr::Identifier(identifier) => Ok(Some(IrValue::Identifier(identifier.name.clone()))),

            Expr::Return(ret) => {
                let lowered = match &ret.value {
                    Some(expr) => self.lower_expr(expr, instructions)?,
                    None => None,
                };

                instructions.push(Instruction::Return(lowered));
                Ok(None)
            }

            Expr::Call(call) => {
                let mut args = Vec::new();

                for arg in &call.args {
                    if let Some(value) = self.lower_expr(arg, instructions)? {
                        args.push(value);
                    }
                }

                instructions.push(Instruction::Call {
                    callee: call.callee.clone(),
                    args,
                });

                Ok(None)
            }

            Expr::PropertyAccess(access) => match access.object.as_ref() {
                Expr::Identifier(identifier) if identifier.name == "self" => {
                    let temp = self.next_temp();

                    instructions.push(Instruction::LoadField {
                        dest: temp.clone(),
                        object: "self".to_string(),
                        field: access.property.clone(),
                    });

                    Ok(Some(IrValue::Identifier(temp)))
                }
                _ => Err(Diagnostic::InvalidSyntax {
                    message: "lowering only supports self.property for now".to_string(),
                }),
            },

            Expr::Assignment(assign) => {
                let (object, field) = match assign.target.as_ref() {
                    Expr::PropertyAccess(access) => match access.object.as_ref() {
                        Expr::Identifier(identifier) if identifier.name == "self" => {
                            ("self".to_string(), access.property.clone())
                        }
                        _ => {
                            return Err(Diagnostic::InvalidSyntax {
                                message:
                                    "lowering only supports assignment to self.property for now"
                                        .to_string(),
                            });
                        }
                    },
                    _ => {
                        return Err(Diagnostic::InvalidSyntax {
                            message: "assignment target must be a property access".to_string(),
                        });
                    }
                };

                let value = self
                    .lower_expr(assign.value.as_ref(), instructions)?
                    .ok_or_else(|| Diagnostic::InvalidSyntax {
                        message: "assignment value did not produce an IR value".to_string(),
                    })?;

                instructions.push(Instruction::StoreField {
                    object,
                    field,
                    value,
                });

                Ok(None)
            }
        }
    }
}
