pub mod assignment;
pub mod identifier;
pub mod literal;
pub mod property_access;

pub use assignment::*;
pub use identifier::*;
pub use literal::*;
pub use property_access::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    StringLiteral(StringLiteral),
    Identifier(Identifier),
    PropertyAccess(PropertyAccess),
    Assignment(Assignment),
}
