# Vanta

Vanta is a language in development written in Rust, organized as a workspace with separate stages for lexing, parsing, semantic analysis, lowering, and LLVM code generation.

## Overview

The current project flow is:

1. Read a `main.vt` file.
2. Validate the package declaration (`pack ...`) at the beginning of the file.
3. Run lexical analysis.
4. Parse into AST.
5. Run semantic checks.
6. Lower into IR.
7. Generate LLVM IR.
8. Compile LLVM IR with `clang`.
9. Run the generated binary.

The main example is in `examples/codegen-llvm`.

## Workspace structure

- `crates/vanta-ast`: AST definitions.
- `crates/vanta-lexer`: lexical analysis.
- `crates/vanta-parser`: language parser.
- `crates/vanta-diagnostics`: diagnostic types and utilities.
- `crates/vanta-sema`: semantic validations.
- `crates/vanta-lowering`: AST to IR conversion.
- `crates/vanta-ir`: IR definitions.
- `crates/vanta-codegen-llvm`: LLVM IR generation.
- `crates/vanta-cli`: executable that compiles and runs the example.

## Requirements

- Rust toolchain with edition 2024 support.
- `clang` installed and available in PATH.
- LLVM 22 compatible with the `llvm22-1` feature used by the `inkwell` crate.

## Dependencies

Rust dependencies are managed by Cargo and are downloaded automatically when you run the workspace. The main external dependencies required to run the project are:

- `inkwell` for LLVM IR generation.
- `clang` to compile generated LLVM IR into an executable binary.

## How to run the example

Go to the example directory and run:

```bash
cd examples/codegen-llvm
cargo run -p vanta-cli
```

The executable reads `main.vt`, generates `build/main.ll`, compiles the binary to `build/main`, and runs it.

## How to compile

The project does not currently have a separate compile-only command. Compilation happens when you run the CLI:

```bash
cd examples/codegen-llvm
cargo run -p vanta-cli
```

This command runs the full pipeline: lexer, parser, semantic checks, lowering, LLVM IR generation, `clang` invocation, and generated binary execution.

If you only want to inspect the generated output, after running the command above you can open `build/main.ll`.

## Entry point requirements (mandatory)

To execute a `.vt` file with the current CLI, the source file must define this exact entry point structure:

- `pack main` at the top of the file.
- A class named `App`.
- A method `main(): Void` inside `App`.

If one of these elements is missing, execution will fail during semantic checks.

Minimal valid entry point:

```vt
pack main

class App() {
    pub function main(): Void {
        print("Hello, Vanta!")
    }
}
```

## Input example

The `examples/codegen-llvm/main.vt` file currently contains a simple program with an `App` class and a `main` method that prints a message.

```vt
pack main

class App() {
    pub function main(): Void {
        print("Hello, Vanta!")
    }
}
```

This is a minimal example of the current language syntax: a package declaration with `pack`, a class with a `main` method, and a function call.

## Generated output

During execution, the project creates the `build/` directory inside the example directory to store:

- `main.ll`: generated LLVM IR.
- `main`: compiled binary.

## Status

This project is still in development and the structure may change as new language stages are implemented.
