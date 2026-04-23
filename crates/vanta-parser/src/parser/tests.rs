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

#[test]
fn should_parse_class_with_method() {
    let tokens = lex(r#"
        class User(
            priv val name: String,
            priv val em: String
        ) {
            pub function greet(): String {
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.classes.len(), 1);

    let class = &program.classes[0];
    assert_eq!(class.name, "User");
    assert_eq!(class.fields.len(), 2);
    assert_eq!(class.methods.len(), 1);

    let method = &class.methods[0];
    assert_eq!(method.name, "greet");
    assert_eq!(method.visibility, vanta_ast::Visibility::Pub);
    assert_eq!(method.return_type, Some(vanta_ast::Type::String));
    assert_eq!(method.params.len(), 0);
    assert!(method.body.is_empty());
}

#[test]
fn should_parse_method_with_parameter() {
    let tokens = lex(r#"
        class User() {
            pub function setName(value: String): Void {
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let method = &program.classes[0].methods[0];

    assert_eq!(method.name, "setName");
    assert_eq!(method.params.len(), 1);
    assert_eq!(method.params[0].name, "value");
    assert_eq!(method.params[0].ty, vanta_ast::Type::String);
    assert_eq!(method.return_type, Some(vanta_ast::Type::Void));
}

#[test]
fn should_parse_method_body_with_assignment() {
    let tokens = lex(r#"
        class User() {
            pub function test(): Void {
                user.email = "abc"
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let method = &program.classes[0].methods[0];

    assert_eq!(method.body.len(), 1);

    match &method.body[0] {
        vanta_ast::Expr::Assignment(_) => {}
        _ => panic!("expected assignment expression"),
    }
}

#[test]
fn should_parse_property_access() {
    let tokens = lex(r#"
        class User() {
            pub function test(): Void {
                user.email
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::PropertyAccess(_) => {}
        _ => panic!("expected property access"),
    }
}

#[test]
fn should_parse_return_without_value() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                return
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Return(ret) => {
            assert!(ret.value.is_none());
        }
        _ => panic!("expected return expression"),
    }
}

#[test]
fn should_parse_return_with_string_value() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                return "Hello"
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Return(ret) => {
            assert!(ret.value.is_some());
        }
        _ => panic!("expected return expression"),
    }
}
