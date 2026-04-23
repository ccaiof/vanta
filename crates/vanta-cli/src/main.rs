use std::fs;

use vanta_lexer::lex;
use vanta_lowering::Lowerer;
use vanta_parser::Parser;
use vanta_sema::{check_assignments, check_entrypoint, check_return_types, check_returns};

fn main() {
    let source = fs::read_to_string("main.vt").expect("failed to read main.vt");

    let tokens = lex(&source).expect("lexer error");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parser error");

    check_entrypoint(&program).expect("semantic error: invalid entrypoint");
    check_returns(&program).expect("semantic error: invalid return structure");
    check_return_types(&program).expect("semantic error: invalid return type");
    check_assignments(&program).expect("semantic error: invalid assignment");

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).expect("lowering error");

    println!("{:#?}", ir);
}
