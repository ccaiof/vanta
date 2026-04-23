use std::fs;
use std::path::Path;

use vanta_ast::Program;
use vanta_lexer::lex;
use vanta_parser::Parser;

pub fn parse_program_from_file(path: &Path) -> Program {
    let source = fs::read_to_string(path)
        .expect("failed to read main.vt");

    let tokens = lex(&source)
        .expect("lexer error");

    let mut parser = Parser::new(tokens);

    parser
        .parse_program()
        .expect("parser error")
}