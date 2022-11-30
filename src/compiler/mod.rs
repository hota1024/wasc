use crate::{
    ast::{
        item::{item_fn::ItemFn, Item},
        lit::lit_ident::LitIdent,
        module::Module,
        ty::Ty,
    },
    sexpr::{self, s_list, s_string, s_symbol},
};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_module(self: &Self, module: Module) -> sexpr::Expr {
        let mut items = vec![];
        items.push(s_symbol!("module"));

        for item in module.items {
            items.push(self.compile_item(item));
        }

        s_list!(items)
    }

    fn compile_item(self: &Self, item: Item) -> sexpr::Expr {
        match item {
            Item::ItemFn(item_fn) => self.compile_item_fn(item_fn),
        }
    }

    fn compile_item_fn(self: &Self, item_fn: ItemFn) -> sexpr::Expr {
        let mut items = vec![];

        items.push(s_symbol!("func"));
        items.push(self.compile_ident(item_fn.name.clone()));

        if item_fn.exported {
            items.push(s_list!(
                s_symbol!("export"),
                self.compile_ident_string(item_fn.name.clone())
            ));
        }

        // let mut params = vec![];

        for param in item_fn.params {
            items.push(s_list!(vec![
                s_symbol!("param"),
                self.compile_ident(param.name),
                self.compile_ty(param.ty),
            ]))
        }

        // items.push(s_list_items!(params));

        items.push(s_list!(vec![
            s_symbol!("result"),
            self.compile_ty(item_fn.ret_ty),
        ]));

        s_list!(items)
    }

    fn compile_ty(self: &Self, ty: Ty) -> sexpr::Expr {
        match ty {
            Ty::TyInt64 => s_symbol!("i64"),
            Ty::TyInt32 => s_symbol!("i32"),
            Ty::TyFloat64 => s_symbol!("f64"),
            Ty::TyFloat32 => s_symbol!("f32"),
        }
    }

    fn compile_ident(self: &Self, ident: LitIdent) -> sexpr::Expr {
        s_symbol!(format!("${}", ident.ident))
    }

    fn compile_ident_string(self: &Self, ident: LitIdent) -> sexpr::Expr {
        s_string!(format!("{}", ident.ident))
    }
}
