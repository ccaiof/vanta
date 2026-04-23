use crate::LlvmCodegen;
use inkwell::context::Context;
use vanta_ir::{Instruction, IrFunction, IrModule, IrValue};

#[test]
fn should_codegen_print_call() {
    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "print_module");

    let ir_module = IrModule {
        functions: vec![IrFunction {
            name: "App.main".to_string(),
            instructions: vec![
                Instruction::Call {
                    callee: "print".to_string(),
                    args: vec![IrValue::StringLiteral("Hello, Vanta!".to_string())],
                },
                Instruction::Return(None),
            ],
        }],
    };

    let result = codegen.codegen_module(&ir_module);

    assert!(result.is_ok());

    let llvm_ir = codegen.module.print_to_string().to_string();

    assert!(llvm_ir.contains("declare i32 @printf(ptr, ...)"));
    assert!(llvm_ir.contains("define void @App.main()"));
    assert!(llvm_ir.contains("call i32 (ptr, ...) @printf"));
    assert!(llvm_ir.contains("ret void"));
}
