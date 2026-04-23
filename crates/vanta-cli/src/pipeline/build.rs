use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn write_llvm_ir(build_dir: &Path, llvm_ir: &str) -> PathBuf {
    if !build_dir.exists() {
        fs::create_dir_all(build_dir).expect("failed to create build directory");
    }

    let ll_path = build_dir.join("main.ll");

    fs::write(&ll_path, llvm_ir).expect("failed to write build/main.ll");

    ll_path
}

pub fn compile_with_clang(ll_path: &Path, output_path: &Path) {
    let clang_status = Command::new("clang")
        .arg(ll_path)
        .arg("-o")
        .arg(output_path)
        .status()
        .expect("failed to execute clang");

    if !clang_status.success() {
        panic!("clang failed to compile LLVM IR");
    }
}

pub fn run_binary(path: &Path) {
    let run_status = Command::new(path)
        .status()
        .expect("failed to execute compiled binary");

    if !run_status.success() {
        panic!("compiled binary exited with error");
    }
}
