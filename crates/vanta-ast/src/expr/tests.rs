use super::*;

#[test]
fn should_create_string_literal_expression() {
    let expr = Expr::StringLiteral(StringLiteral {
        value: "Olá".to_string(),
    });

    assert_eq!(
        expr,
        Expr::StringLiteral(StringLiteral {
            value: "Olá".to_string(),
        })
    );
}

#[test]
fn should_create_identifier_expression() {
    let expr = Expr::Identifier(Identifier {
        name: "user".to_string(),
    });

    assert_eq!(
        expr,
        Expr::Identifier(Identifier {
            name: "user".to_string(),
        })
    );
}

#[test]
fn should_create_property_access_expression() {
    let expr = Expr::PropertyAccess(PropertyAccess {
        object: Box::new(Expr::Identifier(Identifier {
            name: "user".to_string(),
        })),
        property: "email".to_string(),
    });

    assert_eq!(
        expr,
        Expr::PropertyAccess(PropertyAccess {
            object: Box::new(Expr::Identifier(Identifier {
                name: "user".to_string(),
            })),
            property: "email".to_string(),
        })
    );
}

#[test]
fn should_create_assignment_expression() {
    let expr = Expr::Assignment(Assignment {
        target: Box::new(Expr::PropertyAccess(PropertyAccess {
            object: Box::new(Expr::Identifier(Identifier {
                name: "user".to_string(),
            })),
            property: "email".to_string(),
        })),
        value: Box::new(Expr::StringLiteral(StringLiteral {
            value: "novo@email.com".to_string(),
        })),
    });

    assert_eq!(
        expr,
        Expr::Assignment(Assignment {
            target: Box::new(Expr::PropertyAccess(PropertyAccess {
                object: Box::new(Expr::Identifier(Identifier {
                    name: "user".to_string(),
                })),
                property: "email".to_string(),
            })),
            value: Box::new(Expr::StringLiteral(StringLiteral {
                value: "novo@email.com".to_string(),
            })),
        })
    );
}