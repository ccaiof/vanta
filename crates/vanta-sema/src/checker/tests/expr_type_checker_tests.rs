use crate::{TypeContext, check_return_types, infer_expr_type};
use vanta_ast::{
    Call, ClassDecl, Expr, FieldDecl, FunctionDecl, Identifier, Mutability, Param, Program,
    PropertyAccess, ReturnExpr, StringLiteral, Type, Visibility,
};

#[test]
fn should_infer_string_literal_type() {
    let expr = Expr::StringLiteral(StringLiteral {
        value: "Hello".to_string(),
    });

    let context = TypeContext {
        class_name: "App",
        fields: &[],
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert_eq!(result.unwrap(), Type::String);
}

#[test]
fn should_infer_identifier_type_from_method_params() {
    let expr = Expr::Identifier(Identifier {
        name: "name".to_string(),
    });

    let params = vec![Param {
        name: "name".to_string(),
        ty: Type::String,
    }];

    let context = TypeContext {
        class_name: "App",
        fields: &[],
        params: &params,
    };

    let result = infer_expr_type(&expr, &context);

    assert_eq!(result.unwrap(), Type::String);
}

#[test]
fn should_fail_when_identifier_is_unknown() {
    let expr = Expr::Identifier(Identifier {
        name: "name".to_string(),
    });

    let context = TypeContext {
        class_name: "App",
        fields: &[],
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: type inference for identifier 'name' is not implemented yet"
    );
}

#[test]
fn should_accept_string_method_returning_param_identifier() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![vanta_ast::Param {
                    name: "name".to_string(),
                    ty: Type::String,
                }],
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

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_returned_param_type_does_not_match_method_type() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "greet".to_string(),
                params: vec![vanta_ast::Param {
                    name: "age".to_string(),
                    ty: Type::Int,
                }],
                return_type: Some(Type::String),
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::Identifier(vanta_ast::Identifier {
                        name: "age".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let result = check_return_types(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: method 'User.greet' returns String but found Int"
    );
}

#[test]
fn should_infer_print_call_expression_as_void() {
    let expr = Expr::Call(Call {
        callee: "print".to_string(),
        args: vec![Expr::StringLiteral(StringLiteral {
            value: "Hello".to_string(),
        })],
    });

    let context = TypeContext {
        class_name: "App",
        fields: &[],
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert_eq!(result.unwrap(), Type::Void);
}

#[test]
fn should_infer_self_property_access_type() {
    let expr = Expr::PropertyAccess(PropertyAccess {
        object: Box::new(Expr::Identifier(Identifier {
            name: "self".to_string(),
        })),
        property: "name".to_string(),
    });

    let fields = vec![FieldDecl {
        visibility: Visibility::Priv,
        mutability: Mutability::Val,
        name: "name".to_string(),
        ty: Type::String,
    }];

    let context = TypeContext {
        class_name: "User",
        fields: &fields,
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert_eq!(result.unwrap(), Type::String);
}

#[test]
fn should_fail_when_self_property_does_not_exist() {
    let expr = Expr::PropertyAccess(PropertyAccess {
        object: Box::new(Expr::Identifier(Identifier {
            name: "self".to_string(),
        })),
        property: "email".to_string(),
    });

    let fields = vec![FieldDecl {
        visibility: Visibility::Priv,
        mutability: Mutability::Val,
        name: "name".to_string(),
        ty: Type::String,
    }];

    let context = TypeContext {
        class_name: "User",
        fields: &fields,
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: property 'email' was not found in class 'User'"
    );
}

#[test]
fn should_fail_when_property_access_is_not_self_property() {
    let expr = Expr::PropertyAccess(PropertyAccess {
        object: Box::new(Expr::Identifier(Identifier {
            name: "user".to_string(),
        })),
        property: "name".to_string(),
    });

    let context = TypeContext {
        class_name: "User",
        fields: &[],
        params: &[],
    };

    let result = infer_expr_type(&expr, &context);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: type inference for property access is only implemented for self.property"
    );
}
