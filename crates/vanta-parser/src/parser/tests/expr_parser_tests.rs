use crate::parser::Parser;
use vanta_lexer::lex;

#[test]
fn should_parse_method_body_with_assignment() {
    let tokens = lex(r#"
        class User() {
            pub function test(): Void {
                user.email = "abc"
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let method = &program.classes[0].methods[0];

    assert_eq!(method.body.len(), 1);

    match &method.body[0] {
        vanta_ast::Expr::Assignment(_) => {}
        _ => panic!("expected assignment expression"),
    }
}

#[test]
fn should_parse_property_access() {
    let tokens = lex(r#"
        class User() {
            pub function test(): Void {
                user.email
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::PropertyAccess(_) => {}
        _ => panic!("expected property access"),
    }
}

#[test]
fn should_parse_return_without_value() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                return
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Return(ret) => {
            assert!(ret.value.is_none());
        }
        _ => panic!("expected return expression"),
    }
}

#[test]
fn should_parse_return_with_string_value() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                return "Hello"
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Return(ret) => {
            assert!(ret.value.is_some());
        }
        _ => panic!("expected return expression"),
    }
}

#[test]
fn should_parse_function_call_expression() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                print("Hello, Vanta!")
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Call(call) => {
            assert_eq!(call.callee, "print");
            assert_eq!(call.args.len(), 1);
        }
        _ => panic!("expected call expression"),
    }
}

#[test]
fn should_parse_function_call_with_multiple_arguments() {
    let tokens = lex(r#"
        class App() {
            pub function main(): Void {
                log("a", "b")
            }
        }
        "#)
    .unwrap();

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let expr = &program.classes[0].methods[0].body[0];

    match expr {
        vanta_ast::Expr::Call(call) => {
            assert_eq!(call.callee, "log");
            assert_eq!(call.args.len(), 2);
        }
        _ => panic!("expected call expression"),
    }
}
