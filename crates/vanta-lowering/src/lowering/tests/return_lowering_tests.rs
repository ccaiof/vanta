use crate::Lowerer;
use vanta_ast::{
    ClassDecl, Expr, FunctionDecl, Program, ReturnExpr, StringLiteral, Type, Visibility,
};
use vanta_ir::{Instruction, IrValue};

#[test]
fn should_lower_return_string_literal() {
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
                body: vec![Expr::Return(ReturnExpr {
                    value: Some(Box::new(Expr::StringLiteral(StringLiteral {
                        value: "Hello".to_string(),
                    }))),
                })],
            }],
        }],
    };

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).unwrap();

    assert_eq!(ir.functions.len(), 1);
    assert_eq!(
        ir.functions[0].instructions[0],
        Instruction::Return(Some(IrValue::StringLiteral("Hello".to_string())))
    );
}
