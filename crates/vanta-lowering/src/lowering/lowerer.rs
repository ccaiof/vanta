use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;
use vanta_ir::IrModule;

pub struct Lowerer {
    temp_counter: usize,
}

impl Lowerer {
    pub fn new() -> Self {
        Self { temp_counter: 0 }
    }

    pub(crate) fn next_temp(&mut self) -> String {
        let name = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    pub fn lower_program(&mut self, program: &Program) -> Result<IrModule, Diagnostic> {
        self.lower_module(program)
    }
}
