use crate::check_uniqueness;
use vanta_ast::{ClassDecl, FunctionDecl, Program, Type, Visibility};

#[test]
fn should_fail_when_class_name_is_duplicated() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![
            ClassDecl {
                name: "User".to_string(),
                fields: vec![],
                methods: vec![],
            },
            ClassDecl {
                name: "User".to_string(),
                fields: vec![],
                methods: vec![],
            },
        ],
    };

    let result = check_uniqueness(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: class 'User' is already defined"
    );
}

#[test]
fn should_fail_when_method_name_is_duplicated() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![
                FunctionDecl {
                    visibility: Visibility::Pub,
                    name: "greet".to_string(),
                    params: vec![],
                    return_type: Some(Type::String),
                    body: vec![],
                },
                FunctionDecl {
                    visibility: Visibility::Pub,
                    name: "greet".to_string(),
                    params: vec![],
                    return_type: Some(Type::String),
                    body: vec![],
                },
            ],
        }],
    };

    let result = check_uniqueness(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'User.greet' is already defined"
    );
}
