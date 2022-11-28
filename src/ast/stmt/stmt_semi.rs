use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtSemi {
    pub expr: Box<Expr>,
}
