use vanta_ast::Program;
use vanta_ir::IrModule;
use vanta_lowering::Lowerer;

pub fn lower_program(program: &Program) -> IrModule {
    let mut lowerer = Lowerer::new();

    lowerer.lower_program(program).expect("lowering error")
}
