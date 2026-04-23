use vanta_ast::Program;
use vanta_diagnostics::Diagnostic;
use vanta_ir::IrModule;

use crate::lowering::lowerer::Lowerer;

impl Lowerer {
    pub(crate) fn lower_module(&mut self, program: &Program) -> Result<IrModule, Diagnostic> {
        let mut functions = Vec::new();

        for class in &program.classes {
            for method in &class.methods {
                functions.push(self.lower_function(&class.name, method)?);
            }
        }

        Ok(IrModule { functions })
    }
}
