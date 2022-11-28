use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtReturn {
    pub expr: Option<Expr>,
}
