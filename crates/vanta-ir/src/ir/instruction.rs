use crate::IrValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Return(Option<IrValue>),
    Call {
        callee: String,
        args: Vec<IrValue>,
    },
    LoadField {
        dest: String,
        object: String,
        field: String,
    },
    StoreField {
        object: String,
        field: String,
        value: IrValue,
    },
}
