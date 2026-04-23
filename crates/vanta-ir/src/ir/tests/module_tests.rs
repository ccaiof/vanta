use crate::{IrFunction, IrModule};

#[test]
fn should_create_ir_module() {
    let module = IrModule {
        functions: vec![IrFunction {
            name: "App.main".to_string(),
            instructions: vec![],
        }],
    };

    assert_eq!(module.functions.len(), 1);
    assert_eq!(module.functions[0].name, "App.main");
}
