use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct StmtReturn {
    pub expr: Option<Expr>,
}
