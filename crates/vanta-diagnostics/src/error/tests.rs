use super::*;

#[test]
fn should_create_unexpected_token_diagnostic() {
    let diagnostic = Diagnostic::UnexpectedToken {
        expected: "identifier".to_string(),
        found: "RBrace".to_string(),
    };

    assert_eq!(
        diagnostic,
        Diagnostic::UnexpectedToken {
            expected: "identifier".to_string(),
            found: "RBrace".to_string(),
        }
    );

    assert_eq!(
        diagnostic.to_string(),
        "unexpected token: expected identifier, found RBrace"
    );
}

#[test]
fn should_create_unexpected_eof_diagnostic() {
    let diagnostic = Diagnostic::UnexpectedEof;

    assert_eq!(diagnostic, Diagnostic::UnexpectedEof);
    assert_eq!(diagnostic.to_string(), "unexpected end of input");
}

#[test]
fn should_create_invalid_syntax_diagnostic() {
    let diagnostic = Diagnostic::InvalidSyntax {
        message: "setter is not allowed for val property".to_string(),
    };

    assert_eq!(
        diagnostic,
        Diagnostic::InvalidSyntax {
            message: "setter is not allowed for val property".to_string(),
        }
    );

    assert_eq!(
        diagnostic.to_string(),
        "invalid syntax: setter is not allowed for val property"
    );
}
