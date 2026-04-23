use crate::{TypeContext, check_return_types, infer_expr_type};
use vanta_ast::{
    ClassDecl, Expr, FunctionDecl, Identifier, Param, Program, ReturnExpr, StringLiteral, Type,
    Visibility,
};

#[test]
fn should_infer_string_literal_type() {
    let expr = Expr::StringLiteral(StringLiteral {
        value: "Hello".to_string(),
    });

    let context = TypeContext { params: &[] };

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

    let context = TypeContext { params: &params };

    let result = infer_expr_type(&expr, &context);

    assert_eq!(result.unwrap(), Type::String);
}

#[test]
fn should_fail_when_identifier_is_unknown() {
    let expr = Expr::Identifier(Identifier {
        name: "name".to_string(),
    });

    let context = TypeContext { params: &[] };

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
