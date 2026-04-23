use vanta_diagnostics::Diagnostic;
use vanta_ir::Instruction;

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

            _ => Err(Diagnostic::InvalidSyntax {
                message: "LLVM codegen for this instruction is not implemented yet".to_string(),
            }),
        }
    }
}
