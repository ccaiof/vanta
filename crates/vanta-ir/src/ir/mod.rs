pub mod function;
pub mod instruction;
pub mod module;
pub mod value;

pub use function::*;
pub use instruction::*;
pub use module::*;
pub use value::*;

#[cfg(test)]
mod tests;
