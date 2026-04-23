use vanta_diagnostics::Diagnostic;
use vanta_ir::IrModule;

use crate::codegen::codegen::LlvmCodegen;

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn codegen_module(&self, ir_module: &IrModule) -> Result<(), Diagnostic> {
        for function in &ir_module.functions {
            self.codegen_function(function)?;
        }

        Ok(())
    }
}
