use vanta_ast::{Call, ClassDecl, Expr, FunctionDecl, Program, StringLiteral, Type, Visibility};
use vanta_ir::{Instruction, IrValue};
use crate::Lowerer;

#[test]
fn should_lower_call_expression() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "App".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "main".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Call(Call {
                    callee: "print".to_string(),
                    args: vec![Expr::StringLiteral(StringLiteral {
                        value: "Hello".to_string(),
                    })],
                })],
            }],
        }],
    };

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).unwrap();

    assert_eq!(
        ir.functions[0].instructions[0],
        Instruction::Call {
            callee: "print".to_string(),
            args: vec![IrValue::StringLiteral("Hello".to_string())],
        }
    );
}
