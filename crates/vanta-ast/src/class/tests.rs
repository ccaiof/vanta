use super::*;
use crate::{Expr, FieldDecl, FunctionDecl, Mutability, StringLiteral, Type, Visibility};

#[test]
fn should_create_a_class_declaration() {
    let class = ClassDecl {
        name: "User".to_string(),
        fields: vec![
            FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Val,
                name: "name".to_string(),
                ty: Type::String,
            },
            FieldDecl {
                visibility: Visibility::Priv,
                mutability: Mutability::Mut,
                name: "email".to_string(),
                ty: Type::String,
            },
        ],
        methods: vec![FunctionDecl {
            visibility: Visibility::Pub,
            name: "greet".to_string(),
            params: vec![],
            return_type: Some(Type::String),
            body: vec![Expr::StringLiteral(StringLiteral {
                value: "Olá, ${name}".to_string(),
            })],
        }],
    };

    assert_eq!(class.name, "User");
    assert_eq!(class.fields.len(), 2);
    assert_eq!(class.methods.len(), 1);

    let name_field = &class.fields[0];
    assert_eq!(name_field.name, "name");
    assert_eq!(name_field.visibility, Visibility::Priv);
    assert_eq!(name_field.mutability, Mutability::Val);
    assert_eq!(name_field.ty, Type::String);

    let method = &class.methods[0];
    assert_eq!(method.name, "greet");
    assert_eq!(method.visibility, Visibility::Pub);
    assert_eq!(method.return_type, Some(Type::String));
    assert_eq!(method.body.len(), 1);
}
