use vanta_ast::{ClassDecl, FunctionDecl, Program, Type, Visibility};

use crate::check_entrypoint;

#[test]
fn should_accept_valid_app_main_entrypoint() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "main".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![],
            }],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_app_class_does_not_exist() {
    let program = Program { pack: vanta_ast::PackDecl { name: "test".to_string() }, imports: vec![], classes: vec![] };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint class 'App' was not found"
    );
}

#[test]
fn should_fail_when_main_method_does_not_exist() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint method 'App.main' was not found"
    );
}

#[test]
fn should_fail_when_main_is_not_public() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Priv,
                name: "main".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![],
            }],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint method 'App.main' must be public"
    );
}

#[test]
fn should_fail_when_main_has_parameters() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "main".to_string(),
                params: vec![vanta_ast::Param {
                    name: "value".to_string(),
                    ty: Type::String,
                }],
                return_type: Some(Type::Void),
                body: vec![],
            }],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint method 'App.main' must not have parameters"
    );
}

#[test]
fn should_fail_when_main_does_not_return_void() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "main".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![],
            }],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint method 'App.main' must return Void"
    );
}

#[test]
fn should_fail_when_app_class_is_duplicated() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![
            ClassDecl {
                name: "App".to_string(),
                fields: vec![],
                methods: vec![],
            },
            ClassDecl {
                name: "App".to_string(),
                fields: vec![],
                methods: vec![],
            },
        ],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint class 'App' must be unique"
    );
}

#[test]
fn should_fail_when_main_method_is_duplicated() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![
                FunctionDecl {
                    visibility: Visibility::Pub,
                    name: "main".to_string(),
                    params: vec![],
                    return_type: Some(Type::Void),
                    body: vec![],
                },
                FunctionDecl {
                    visibility: Visibility::Pub,
                    name: "main".to_string(),
                    params: vec![],
                    return_type: Some(Type::Void),
                    body: vec![],
                },
            ],
        }],
    };

    let result = check_entrypoint(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: entrypoint method 'App.main' must be unique"
    );
}
