use crate::ast::ty::Ty;

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprAs {
    pub expr: Box<Expr>,
    pub ty: Ty,
}
