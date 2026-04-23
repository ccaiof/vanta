use crate::infer_expr_type;
use vanta_ast::{Expr, Identifier, StringLiteral, Type};

#[test]
fn should_infer_string_literal_type() {
    let expr = Expr::StringLiteral(StringLiteral {
        value: "Hello".to_string(),
    });

    let result = infer_expr_type(&expr);

    assert_eq!(result.unwrap(), Type::String);
}

#[test]
fn should_fail_to_infer_identifier_type_for_now() {
    let expr = Expr::Identifier(Identifier {
        name: "name".to_string(),
    });

    let result = infer_expr_type(&expr);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: type inference for identifier 'name' is not implemented yet"
    );
}
