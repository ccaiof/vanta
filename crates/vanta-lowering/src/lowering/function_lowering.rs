use vanta_ast::FunctionDecl;
use vanta_diagnostics::Diagnostic;
use vanta_ir::IrFunction;

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

        Ok(IrFunction {
            name: format!("{}.{}", class_name, method.name),
            instructions,
        })
    }
}
