use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    // TODO: Add more operators
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprUnary {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}
