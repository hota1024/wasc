use super::{expr_block::ExprBlock, Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct ExprIf {
    pub cond: Box<Expr>,
    pub then_branch: ExprBlock,
    pub else_branch: Option<Box<Expr>>,
}
