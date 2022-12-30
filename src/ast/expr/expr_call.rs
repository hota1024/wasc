use crate::ast::lit::lit_ident::LitIdent;

use super::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct ExprCall {
    pub fn_name: LitIdent,
    pub args: Vec<Expr>,
}
