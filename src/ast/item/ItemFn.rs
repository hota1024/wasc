use crate::ast::{expr::ExprBlock::ExprBlock, lit::LitIdent::LitIdent, ty::Ty};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemFn {
    pub exported: bool,
    pub name: LitIdent,
    pub params: Vec<FnParam>,
    pub ret_ty: Ty,
    pub body: ExprBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnParam {
    name: LitIdent,
    ty: Ty,
}
