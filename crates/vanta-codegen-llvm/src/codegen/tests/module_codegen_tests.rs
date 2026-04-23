use crate::LlvmCodegen;
use inkwell::context::Context;
use vanta_ir::{Instruction, IrFunction, IrModule};

#[test]
fn should_codegen_module_with_single_void_function() {
    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "main_module");

    let ir_module = IrModule {
        functions: vec![IrFunction {
            name: "App.main".to_string(),
            instructions: vec![Instruction::Return(None)],
        }],
    };

    let result = codegen.codegen_module(&ir_module);

    assert!(result.is_ok());

    let llvm_ir = codegen.module.print_to_string().to_string();
    assert!(llvm_ir.contains("define void @App.main()"));
}
