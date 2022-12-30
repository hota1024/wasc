use crate::ast::{expr::Expr, lit::lit_ident::LitIdent, ty::Ty};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtLet {
    pub name: LitIdent,
    pub ty: Option<Ty>,
    pub initializer: Expr,
}
