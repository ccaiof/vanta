use crate::parser::Parser;
use vanta_lexer::lex;

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
