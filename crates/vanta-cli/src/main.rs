use std::env;
use std::fs;

use vanta_lexer::lex;
use vanta_parser::Parser;
use vanta_sema::{check_assignments, check_entrypoint, check_return_types, check_returns};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: vanta <file.vt>");
        return;
    }

    let source = fs::read_to_string("main.vt").expect("main.vt not found");

    let tokens = lex(&source).expect("lexer error");

    let mut parser = Parser::new(tokens);

    let program = parser.parse_program().expect("parser error");

    check_entrypoint(&program).expect("semantic error");
    check_returns(&program).expect("semantic error: invalid return usage");
    check_return_types(&program).expect("semantic error: invalid return type");
    check_assignments(&program).expect("semantic error: invalid assignment");

    println!("{:#?}", program);
}
