use crate::ast::stmt::Stmt;

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprBlock {
    pub stmts: Vec<Stmt>,
    pub last_expr: Option<Box<Expr>>,
}
