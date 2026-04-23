pub mod builtin_type_checker;
pub mod entrypoint_checker;
pub mod expr_type_checker;
pub mod return_checker;
pub mod return_type_checker;

pub use builtin_type_checker::*;
pub use entrypoint_checker::*;
pub use expr_type_checker::*;
pub use return_checker::*;
pub use return_type_checker::*;

#[cfg(test)]
mod tests;
