use crate::{Instruction, IrValue};

#[test]
fn should_create_return_instruction() {
    let instr = Instruction::Return(Some(IrValue::StringLiteral("Hello".to_string())));

    match instr {
        Instruction::Return(Some(IrValue::StringLiteral(value))) => {
            assert_eq!(value, "Hello");
        }
        _ => panic!("expected return with string literal"),
    }
}
