use inkwell::values::BasicMetadataValueEnum;
use vanta_diagnostics::Diagnostic;
use vanta_ir::{Instruction, IrValue};

use crate::codegen::codegen::LlvmCodegen;

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn codegen_instruction(&self, instruction: &Instruction) -> Result<(), Diagnostic> {
        match instruction {
            Instruction::Return(None) => {
                self.builder
                    .build_return(None)
                    .map_err(|err| Diagnostic::InvalidSyntax {
                        message: format!("failed to build void return: {err}"),
                    })?;

                Ok(())
            }

            Instruction::Call { callee, args } => self.codegen_call(callee, args),

            _ => Err(Diagnostic::InvalidSyntax {
                message: "LLVM codegen for this instruction is not implemented yet".to_string(),
            }),
        }
    }

    fn codegen_call(&self, callee: &str, args: &[IrValue]) -> Result<(), Diagnostic> {
        match callee {
            "print" => self.codegen_print(args),
            _ => Err(Diagnostic::InvalidSyntax {
                message: format!("LLVM codegen for call '{callee}' is not implemented yet"),
            }),
        }
    }

    fn codegen_print(&self, args: &[IrValue]) -> Result<(), Diagnostic> {
        let printf =
            self.module
                .get_function("printf")
                .ok_or_else(|| Diagnostic::InvalidSyntax {
                    message: "printf function was not declared".to_string(),
                })?;

        if args.len() != 1 {
            return Err(Diagnostic::InvalidSyntax {
                message: format!("print expects exactly 1 argument, found {}", args.len()),
            });
        }

        let arg = self.codegen_ir_value(&args[0])?;

        let printf_args: &[BasicMetadataValueEnum<'ctx>] = &[arg.into()];

        self.builder
            .build_call(printf, printf_args, "printf_call")
            .map_err(|err| Diagnostic::InvalidSyntax {
                message: format!("failed to build printf call: {err}"),
            })?;

        Ok(())
    }

    fn codegen_ir_value(
        &self,
        value: &IrValue,
    ) -> Result<inkwell::values::BasicValueEnum<'ctx>, Diagnostic> {
        match value {
            IrValue::StringLiteral(text) => {
                let format_text = format!("{text}\n");
                let ptr = self
                    .builder
                    .build_global_string_ptr(&format_text, "str")
                    .map_err(|err| Diagnostic::InvalidSyntax {
                        message: format!("failed to build global string: {err}"),
                    })?;

                Ok(ptr.as_pointer_value().into())
            }

            IrValue::Identifier(name) => Err(Diagnostic::InvalidSyntax {
                message: format!("LLVM codegen for identifier '{name}' is not implemented yet"),
            }),
        }
    }
}
