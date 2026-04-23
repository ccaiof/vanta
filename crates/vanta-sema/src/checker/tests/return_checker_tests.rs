use vanta_ast::{
    ClassDecl, Expr, FunctionDecl, Program, ReturnExpr, StringLiteral, Type, Visibility,
};

use crate::{check_return_types, check_returns};

#[test]
fn should_accept_void_method_with_empty_return() {
    let program = Program {
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "main".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Return(ReturnExpr { value: None })],
            }],
        }],
    };

    let result = check_returns(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_void_method_returns_value() {
    let program = Program {
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

    let result = check_returns(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'App.main' returns Void and cannot return a value"
    );
}

#[test]
fn should_accept_string_method_with_return_value() {
    let program = Program {
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

    let result = check_returns(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_string_method_has_no_return() {
    let program = Program {
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![],
            }],
        }],
    };

    let result = check_returns(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'User.greet' returns String and must end with return"
    );
}

#[test]
fn should_fail_when_string_method_returns_without_value() {
    let program = Program {
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

    let result = check_returns(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'User.greet' returns String and must return a value"
    );
}

#[test]
fn should_accept_string_method_returning_self_property() {
    let program = Program {
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![vanta_ast::FieldDecl {
                visibility: Visibility::Priv,
                mutability: vanta_ast::Mutability::Val,
                name: "name".to_string(),
                ty: Type::String,
            }],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::PropertyAccess(vanta_ast::PropertyAccess {
                        object: Box::new(Expr::Identifier(vanta_ast::Identifier {
                            name: "self".to_string(),
                        })),
                        property: "name".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_ok());
}
