use vanta_ast::FunctionDecl;
use vanta_diagnostics::Diagnostic;
use vanta_ir::{Instruction, IrFunction};

use crate::lowering::lowerer::Lowerer;

impl Lowerer {
    pub(crate) fn lower_function(
        &mut self,
        class_name: &str,
        method: &FunctionDecl,
    ) -> Result<IrFunction, Diagnostic> {
        let mut instructions = Vec::new();

        for expr in &method.body {
            self.lower_expr(expr, &mut instructions)?;
        }

        if instructions.is_empty() || !matches!(instructions.last(), Some(Instruction::Return(_))) {
            instructions.push(Instruction::Return(None));
        }

        Ok(IrFunction {
            name: format!("{}.{}", class_name, method.name),
            instructions,
        })
    }
}
