use crate::LlvmCodegen;
use inkwell::context::Context;
use vanta_ir::{Instruction, IrFunction};

#[test]
fn should_codegen_void_function_with_return() {
    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "test_module");

    let function = IrFunction {
        name: "App.main".to_string(),
        instructions: vec![Instruction::Return(None)],
    };

    let result = codegen.codegen_function(&function);

    assert!(result.is_ok());

    let llvm_ir = codegen.module.print_to_string().to_string();
    assert!(llvm_ir.contains("define void @App.main()"));
    assert!(llvm_ir.contains("ret void"));
}
