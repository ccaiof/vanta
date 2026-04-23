pub mod expr_lowering;
pub mod function_lowering;
pub mod lowerer;
pub mod module_lowering;

pub use lowerer::*;

#[cfg(test)]
mod tests;
