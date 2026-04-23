use vanta_ast::{
    ClassDecl, Expr, FunctionDecl, Program, ReturnExpr, StringLiteral, Type, Visibility,
};

use crate::check_return_types;

#[test]
fn should_accept_string_method_returning_string_literal() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::StringLiteral(StringLiteral {
                        value: "Hello".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_void_method_returns_string_literal() {
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
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::StringLiteral(StringLiteral {
                        value: "Hello".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'App.main' returns Void and cannot return a value"
    );
}

#[test]
fn should_fail_when_string_method_returns_without_value() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![Expr::Return(ReturnExpr { value: None })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'User.greet' returns String and must return a value"
    );
}

#[test]
fn should_fail_when_string_method_returns_non_string_expression() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::Identifier(vanta_ast::Identifier {
                        name: "name".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: type inference for identifier 'name' is not implemented yet"
    );
}
