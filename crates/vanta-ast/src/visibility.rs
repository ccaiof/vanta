#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Pub,
    Priv,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mutability {
    Val,
    Mut,
}
