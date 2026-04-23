use crate::check_imports;
use vanta_ast::{ImportDecl, PackDecl, Program};

#[test]
fn should_accept_unique_imports() {
    let program = Program {
        pack: PackDecl {
            name: "main".to_string(),
        },
        imports: vec![
            ImportDecl {
                name: "utils".to_string(),
            },
            ImportDecl {
                name: "infra.db".to_string(),
            },
        ],
        classes: vec![],
    };

    let result = check_imports(&program);

    assert!(result.is_ok());
}

#[test]
fn should_fail_when_import_name_is_empty() {
    let program = Program {
        pack: PackDecl {
            name: "main".to_string(),
        },
        imports: vec![ImportDecl {
            name: "".to_string(),
        }],
        classes: vec![],
    };

    let result = check_imports(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: import name cannot be empty"
    );
}

#[test]
fn should_fail_when_import_is_duplicated() {
    let program = Program {
        pack: PackDecl {
            name: "main".to_string(),
        },
        imports: vec![
            ImportDecl {
                name: "utils".to_string(),
            },
            ImportDecl {
                name: "utils".to_string(),
            },
        ],
        classes: vec![],
    };

    let result = check_imports(&program);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "invalid syntax: duplicate import 'utils'"
    );
}
