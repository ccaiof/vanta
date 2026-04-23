pub mod codegen;
pub mod function_codegen;
pub mod instruction_codegen;
pub mod module_codegen;

pub use codegen::*;

#[cfg(test)]
mod tests;
