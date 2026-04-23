use vanta_ast::Program;
use vanta_sema::{
    check_assignments, check_entrypoint, check_imports, check_pack, check_return_types,
    check_returns, check_uniqueness,
};

pub fn run_semantic_checks(program: &Program) {
    check_uniqueness(program).expect("semantic error: duplicate definitions");

    check_pack(program).expect("semantic error: invalid pack");

    check_imports(program).expect("semantic error: invalid imports");

    check_entrypoint(program).expect("semantic error: invalid entrypoint");

    check_returns(program).expect("semantic error: invalid return structure");

    check_return_types(program).expect("semantic error: invalid return type");

    check_assignments(program).expect("semantic error: invalid assignment");
}
