use crate::parser::Parser;
use vanta_lexer::lex;

#[test]
fn should_parse_empty_class() {
    let tokens = lex("class User() {}").unwrap();
    let mut parser = Parser::new(tokens);

    let program = parser.parse_program().unwrap();

    assert_eq!(program.classes.len(), 1);
    assert_eq!(program.classes[0].name, "User");
    assert_eq!(program.classes[0].fields.len(), 0);
    assert_eq!(program.classes[0].methods.len(), 0);
}

#[test]
fn should_fail_when_class_name_is_missing() {
    let tokens = lex("class () {}").unwrap();
    let mut parser = Parser::new(tokens);

    let result = parser.parse_program();

    assert!(result.is_err());
}
