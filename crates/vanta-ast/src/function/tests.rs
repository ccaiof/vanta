use super::*;
use crate::{Expr, StringLiteral, Type, Visibility};

#[test]
fn should_create_a_function_with_body() {
    let function = FunctionDecl {
        visibility: Visibility::Pub,
        name: "greet".to_string(),
        params: vec![],
        return_type: Some(Type::String),
        body: vec![
            Expr::StringLiteral(StringLiteral {
                value: "Olá, ${name}".to_string(),
            }),
        ],
    };

    assert_eq!(function.name, "greet");
    assert_eq!(function.visibility, Visibility::Pub);
    assert_eq!(function.return_type, Some(Type::String));
    assert_eq!(function.params.len(), 0);
    assert_eq!(function.body.len(), 1);

    assert_eq!(
        function.body[0],
        Expr::StringLiteral(StringLiteral {
            value: "Olá, ${name}".to_string(),
        })
    );
}