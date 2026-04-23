use inkwell::{AddressSpace, types::BasicMetadataTypeEnum};
use vanta_diagnostics::Diagnostic;
use vanta_ir::IrModule;

use crate::codegen::codegen::LlvmCodegen;

impl<'ctx> LlvmCodegen<'ctx> {
    pub fn codegen_module(&self, ir_module: &IrModule) -> Result<(), Diagnostic> {
        self.declare_printf();

        for function in &ir_module.functions {
            self.codegen_function(function)?;
        }

        Ok(())
    }

    fn declare_printf(&self) {
        if self.module.get_function("printf").is_some() {
            return;
        }

        let ptr_type = self.context.ptr_type(AddressSpace::default());

        let printf_type = self
            .context
            .i32_type()
            .fn_type(&[BasicMetadataTypeEnum::from(ptr_type)], true);

        self.module.add_function("printf", printf_type, None);
    }
}
