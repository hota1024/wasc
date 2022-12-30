use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct StmtSemi {
    pub expr: Box<Expr>,
}
