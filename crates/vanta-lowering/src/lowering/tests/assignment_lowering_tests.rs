use crate::Lowerer;
use vanta_ast::{
    Assignment, ClassDecl, Expr, FunctionDecl, Identifier, Program, PropertyAccess, StringLiteral,
    Type, Visibility,
};
use vanta_ir::{Instruction, IrValue};

#[test]
fn should_lower_self_property_assignment() {
    let program = Program {
        pack: vanta_ast::PackDecl { name: "test".to_string() },
        imports: vec![],
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "setEmail".to_string(),
                params: vec![],
                return_type: Some(Type::Void),
                body: vec![Expr::Assignment(Assignment {
                    target: Box::new(Expr::PropertyAccess(PropertyAccess {
                        object: Box::new(Expr::Identifier(Identifier {
                            name: "self".to_string(),
                        })),
                        property: "email".to_string(),
                    })),
                    value: Box::new(Expr::StringLiteral(StringLiteral {
                        value: "a@a.com".to_string(),
                    })),
                })],
            }],
        }],
    };

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).unwrap();

    assert_eq!(
        ir.functions[0].instructions[0],
        Instruction::StoreField {
            object: "self".to_string(),
            field: "email".to_string(),
            value: IrValue::StringLiteral("a@a.com".to_string()),
        }
    );
}
