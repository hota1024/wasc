use super::{expr_block::ExprBlock, Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprWhile {
    pub cond: Box<Expr>,
    pub body: ExprBlock,
}
