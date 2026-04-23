use vanta_diagnostics::Diagnostic;
use vanta_ir::IrFunction;

use crate::codegen::codegen::LlvmCodegen;

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn codegen_function(&self, function: &IrFunction) -> Result<(), Diagnostic> {
        let fn_type = self.context.void_type().fn_type(&[], false);
        let llvm_function = self.module.add_function(&function.name, fn_type, None);
        let entry_block = self.context.append_basic_block(llvm_function, "entry");

        self.builder.position_at_end(entry_block);

        for instruction in &function.instructions {
            self.codegen_instruction(instruction)?;
        }

        if let Some(block) = self.builder.get_insert_block() {
            if block.get_terminator().is_none() {
                self.builder
                    .build_return(None)
                    .map_err(|err| Diagnostic::InvalidSyntax {
                        message: format!("failed to build implicit return: {err}"),
                    })?;
            }
        }

        Ok(())
    }
}
