use std::env;
use std::path::Path;

mod pipeline;
mod validation;

use pipeline::{
    compile_with_clang, generate_llvm_ir, lower_program, parse_program_from_file, run_binary,
    run_semantic_checks, write_llvm_ir,
};
use validation::{validate_entry_file, validate_project_context};

fn main() {
    let current_dir = env::current_dir().expect("failed to get current directory");
    let main_file = validate_project_context(&current_dir);
    validate_entry_file(&main_file);

    let program = parse_program_from_file(&main_file);
    run_semantic_checks(&program);

    let ir = lower_program(&program);
    let llvm_ir = generate_llvm_ir(&ir);

    let build_dir = Path::new("build");
    let ll_path = write_llvm_ir(build_dir, &llvm_ir);
    let bin_path = build_dir.join("main");

    compile_with_clang(&ll_path, &bin_path);
    run_binary(&bin_path);
}
