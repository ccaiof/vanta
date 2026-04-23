use crate::check_assignments;
use vanta_ast::{
    Assignment, ClassDecl, Expr, FieldDecl, FunctionDecl, Identifier, Mutability, Program,
    PropertyAccess, StringLiteral, Type, Visibility,
};

#[test]
fn should_accept_assignment_to_mut_string_field() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Mut,
                name: "email".to_string(),
                ty: Type::String,
            }],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "updateEmail".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Assignment(Assignment {
                    target: Box::new(Expr::PropertyAccess(PropertyAccess {
                        object: Box::new(Expr::Identifier(Identifier {
                            name: "self".to_string(),
                        })),
                        property: "email".to_string(),
                    })),
                    value: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "novo@email.com".to_string(),
                    })),
                })],
            }],
        }],
    };

    let result = check_assignments(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_assigning_to_immutable_field() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Val,
                name: "name".to_string(),
                ty: Type::String,
            }],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "rename".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Assignment(Assignment {
                    target: Box::new(Expr::PropertyAccess(PropertyAccess {
                        object: Box::new(Expr::Identifier(Identifier {
                            name: "self".to_string(),
                        })),
                        property: "name".to_string(),
                    })),
                    value: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "Caio".to_string(),
                    })),
                })],
            }],
        }],
    };

    let result = check_assignments(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: property 'User.name' is immutable and cannot receive assignment"
    );
}

#[test]
fn should_fail_when_assignment_value_type_does_not_match_field_type() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Mut,
                name: "age".to_string(),
                ty: Type::Int,
            }],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "updateAge".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Assignment(Assignment {
                    target: Box::new(Expr::PropertyAccess(PropertyAccess {
                        object: Box::new(Expr::Identifier(Identifier {
                            name: "self".to_string(),
                        })),
                        property: "age".to_string(),
                    })),
                    value: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "vinte".to_string(),
                    })),
                })],
            }],
        }],
    };

    let result = check_assignments(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: assignment to 'User.age' expects Int but found String"
    );
}

#[test]
fn should_fail_when_assignment_target_is_not_self_property() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Mut,
                name: "email".to_string(),
                ty: Type::String,
            }],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "updateEmail".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Assignment(Assignment {
                    target: Box::new(Expr::PropertyAccess(PropertyAccess {
                        object: Box::new(Expr::Identifier(Identifier {
                            name: "user".to_string(),
                        })),
                        property: "email".to_string(),
                    })),
                    value: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "novo@email.com".to_string(),
                    })),
                })],
            }],
        }],
    };

    let result = check_assignments(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: assignment in 'User.updateEmail' currently only supports self.property"
    );
}
