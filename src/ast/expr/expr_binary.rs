use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    AssignOp(AssignOp),
    // TODO: Add more operators
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: BinaryOp,
}
