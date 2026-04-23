use crate::check_pack;
use vanta_ast::{PackDecl, Program};

#[test]
fn should_accept_valid_pack() {
    let program = Program {
        pack: PackDecl {
            name: "main".to_string(),
        },
        imports: vec![],
        classes: vec![],
    };

    let result = check_pack(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_pack_name_is_empty() {
    let program = Program {
        pack: PackDecl {
            name: "".to_string(),
        },
        imports: vec![],
        classes: vec![],
    };

    let result = check_pack(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: pack name cannot be empty"
    );
}
