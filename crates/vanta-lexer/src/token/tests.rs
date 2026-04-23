use super::*;

#[test]
fn should_create_keyword_token() {
    let token = Token {
        kind: TokenKind::Class,
    };

    assert_eq!(
        token,
        Token {
            kind: TokenKind::Class,
        }
    );
}

#[test]
fn should_create_identifier_token() {
    let token = Token {
        kind: TokenKind::Identifier("User".to_string()),
    };

    assert_eq!(
        token,
        Token {
            kind: TokenKind::Identifier("User".to_string()),
        }
    );
}

#[test]
fn should_create_string_literal_token() {
    let token = Token {
        kind: TokenKind::StringLiteral("Olá".to_string()),
    };

    assert_eq!(
        token,
        Token {
            kind: TokenKind::StringLiteral("Olá".to_string()),
        }
    );
}
