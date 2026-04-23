use super::*;
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

#[test]
fn should_parse_class_with_fields() {
    let tokens = lex(r#"
        class User(
            priv val name: String,
            priv mut email: String
        ) {}
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.classes.len(), 1);

    let class = &program.classes[0];
    assert_eq!(class.name, "User");
    assert_eq!(class.fields.len(), 2);

    let first = &class.fields[0];
    assert_eq!(first.name, "name");
    assert_eq!(first.visibility, vanta_ast::Visibility::Priv);
    assert_eq!(first.mutability, vanta_ast::Mutability::Val);
    assert_eq!(first.ty, vanta_ast::Type::String);

    let second = &class.fields[1];
    assert_eq!(second.name, "email");
    assert_eq!(second.visibility, vanta_ast::Visibility::Priv);
    assert_eq!(second.mutability, vanta_ast::Mutability::Mut);
    assert_eq!(second.ty, vanta_ast::Type::String);
}

#[test]
fn should_parse_class_with_custom_type_field() {
    let tokens = lex(r#"
        class User(
            priv val profile: Profile
        ) {}
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let field = &program.classes[0].fields[0];
    assert_eq!(field.name, "profile");
    assert_eq!(field.ty, vanta_ast::Type::Custom("Profile".to_string()));
}

#[test]
fn should_parse_fields_with_comma() {
    let tokens = lex("class User(priv val name: String, priv val em: String) {}").unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let class = &program.classes[0];

    assert_eq!(class.fields.len(), 2);
}
