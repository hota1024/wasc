use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    // TODO: Add more operators
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinaryOp,
}
