use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StmtSemi {
    expr: Box<Expr>,
}
