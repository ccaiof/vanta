use crate::infer_builtin_call_type;
use vanta_ast::{Call, Expr, StringLiteral, Type};

#[test]
fn should_infer_print_call_as_void() {
    let call = Call {
        callee: "print".to_string(),
        args: vec![Expr::StringLiteral(StringLiteral {
            value: "Hello".to_string(),
        })],
    };

    let result = infer_builtin_call_type(&call);

    assert_eq!(result.unwrap(), Type::Void);
}

#[test]
fn should_fail_for_unknown_builtin_call() {
    let call = Call {
        callee: "log".to_string(),
        args: vec![],
    };

    let result = infer_builtin_call_type(&call);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: type inference for call 'log' is not implemented yet"
    );
}
