use crate::ast::{lit::lit_ident::LitIdent, ty::Ty};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemImport {
    pub mod_name: LitIdent,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportItem {
    pub kind: ImportItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportItemKind {
    Fn(ImportItemFn),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportItemFn {
    pub name: LitIdent,
    pub params: Vec<ImportItemFnParam>,
    pub ret_ty: Option<Ty>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportItemFnParam {
    pub ty: Ty,
}
