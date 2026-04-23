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

        self.codegen_native_entrypoint(ir_module)?;

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

    fn codegen_native_entrypoint(&self, ir_module: &IrModule) -> Result<(), Diagnostic> {
        let has_app_main = ir_module
            .functions
            .iter()
            .any(|function| function.name == "App.main");

        if !has_app_main {
            return Ok(());
        }

        if self.module.get_function("main").is_some() {
            return Ok(());
        }

        let app_main =
            self.module
                .get_function("App.main")
                .ok_or_else(|| Diagnostic::InvalidSyntax {
                    message: "LLVM function 'App.main' was not generated".to_string(),
                })?;

        let main_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");

        self.builder.position_at_end(entry);

        self.builder
            .build_call(app_main, &[], "call_app_main")
            .map_err(|err| Diagnostic::InvalidSyntax {
                message: format!("failed to build call to App.main: {err}"),
            })?;

        let zero = self.context.i32_type().const_int(0, false);

        self.builder
            .build_return(Some(&zero))
            .map_err(|err| Diagnostic::InvalidSyntax {
                message: format!("failed to build native main return: {err}"),
            })?;

        Ok(())
    }
}
