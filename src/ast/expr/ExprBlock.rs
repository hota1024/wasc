use crate::ast::stmt::Stmt;

use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprBlock {
    stmts: Vec<Stmt>,
    last_expr: Option<Box<Expr>>,
}
