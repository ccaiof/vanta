use super::*;
use crate::TokenKind;

#[test]
fn should_lex_class_declaration_tokens() {
    let tokens = lex("class User").unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Class);
    assert_eq!(tokens[1].kind, TokenKind::Identifier("User".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}

#[test]
fn should_lex_field_declaration_tokens() {
    let tokens = lex("priv val name: String").unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Priv);
    assert_eq!(tokens[1].kind, TokenKind::Val);
    assert_eq!(tokens[2].kind, TokenKind::Identifier("name".to_string()));
    assert_eq!(tokens[3].kind, TokenKind::Colon);
    assert_eq!(tokens[4].kind, TokenKind::StringType);
    assert_eq!(tokens[5].kind, TokenKind::Eof);
}

#[test]
fn should_lex_string_literal_token() {
    let tokens = lex("\"Olá\"").unwrap();

    assert_eq!(tokens[0].kind, TokenKind::StringLiteral("Olá".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn should_return_error_for_unterminated_string() {
    let result = lex("\"Olá");

    assert!(result.is_err());
}

#[test]
fn should_lex_return_keyword() {
    let tokens = lex("return").unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Return);
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn should_lex_function_call_tokens() {
    let tokens = lex(r#"print("Hello")"#).unwrap();

    assert_eq!(tokens[0].kind, TokenKind::Identifier("print".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::LParen);
    assert_eq!(
        tokens[2].kind,
        TokenKind::StringLiteral("Hello".to_string())
    );
    assert_eq!(tokens[3].kind, TokenKind::RParen);
    assert_eq!(tokens[4].kind, TokenKind::Eof);
}
