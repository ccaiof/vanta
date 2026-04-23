use crate::Lowerer;
use vanta_ast::{
    ClassDecl, Expr, FunctionDecl, Identifier, Program, PropertyAccess, Type, Visibility,
};
use vanta_ir::Instruction;

#[test]
fn should_lower_self_property_access() {
    let program = Program {
        classes: vec![ClassDecl {
            name: "User".to_string(),
            fields: vec![],
            methods: vec![FunctionDecl {
                visibility: Visibility::Pub,
                name: "nameValue".to_string(),
                params: vec![],
                return_type: Some(Type::String),
                body: vec![Expr::PropertyAccess(PropertyAccess {
                    object: Box::new(Expr::Identifier(Identifier {
                        name: "self".to_string(),
                    })),
                    property: "name".to_string(),
                })],
            }],
        }],
    };

    let mut lowerer = Lowerer::new();
    let ir = lowerer.lower_program(&program).unwrap();

    assert_eq!(
        ir.functions[0].instructions[0],
        Instruction::LoadField {
            dest: "t0".to_string(),
            object: "self".to_string(),
            field: "name".to_string(),
        }
    );
}
