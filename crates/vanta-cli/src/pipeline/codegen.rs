use inkwell::context::Context;
use vanta_codegen_llvm::LlvmCodegen;
use vanta_ir::IrModule;

pub fn generate_llvm_ir(ir: &IrModule) -> String {
    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "vanta_module");

    codegen
        .codegen_module(ir)
        .expect("llvm codegen error");

    codegen.module.print_to_string().to_string()
}