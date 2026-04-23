use std::fs;
use std::path::Path;
use std::process::Command;

use inkwell::context::Context;
use vanta_codegen_llvm::LlvmCodegen;
use vanta_lexer::lex;
use vanta_lowering::Lowerer;
use vanta_parser::Parser;
use vanta_sema::{
    check_assignments, check_entrypoint, check_return_types, check_returns, check_uniqueness,
};

fn main() {
    let source = fs::read_to_string("main.vt").expect("failed to read main.vt");

    let tokens = lex(&source).expect("lexer error");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parser error");

    check_uniqueness(&program).expect("semantic error: duplicate definitions");
    check_entrypoint(&program).expect("semantic error: invalid entrypoint");
    check_returns(&program).expect("semantic error: invalid return structure");
    check_return_types(&program).expect("semantic error: invalid return type");
    check_assignments(&program).expect("semantic error: invalid assignment");

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).expect("lowering error");

    let context = Context::create();
    let codegen = LlvmCodegen::new(&context, "vanta_module");

    codegen.codegen_module(&ir).expect("llvm codegen error");

    let llvm_ir = codegen.module.print_to_string().to_string();

    let build_dir = Path::new("build");
    if !build_dir.exists() {
        fs::create_dir_all(build_dir).expect("failed to create build directory");
    }

    let ll_path = build_dir.join("main.ll");
    let bin_path = build_dir.join("main");

    fs::write(&ll_path, llvm_ir).expect("failed to write build/main.ll");

    let clang_status = Command::new("clang")
        .arg(&ll_path)
        .arg("-o")
        .arg(&bin_path)
        .status()
        .expect("failed to execute clang");

    if !clang_status.success() {
        panic!("clang failed to compile build/main.ll");
    }

    let run_status = Command::new(&bin_path)
        .status()
        .expect("failed to execute compiled binary");

    if !run_status.success() {
        panic!("compiled binary exited with error");
    }
}
