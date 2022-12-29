pub mod scope;

use std::fmt::Binary;

use crate::{
    ast::{
        expr::{
            expr_binary::{BinaryOp, ExprBinary},
            expr_block::ExprBlock,
            expr_call::ExprCall,
            Expr,
        },
        item::{
            item_fn::ItemFn,
            item_import::{ImportItemFn, ImportItemKind, ItemImport},
            Item,
        },
        lit::{lit_ident::LitIdent, Lit},
        module::Module,
        stmt::{stmt_return::StmtReturn, stmt_semi::StmtSemi, Stmt},
        ty::Ty,
    },
    sexpr::{self, s_expand, s_list, s_string, s_symbol},
};

use self::scope::Scope;

pub struct Compiler {
    scope: Scope,
    last_ret_ty: Ty,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            last_ret_ty: Ty::Void,
        }
    }

    pub fn compile_module(&mut self, module: Module) -> sexpr::Expr {
        let mut items = vec![];
        items.push(s_symbol!("module"));

        for item in module.items {
            items.push(self.compile_item(item));
        }

        s_list!(items)
    }

    fn compile_item(&mut self, item: Item) -> sexpr::Expr {
        match item {
            Item::ItemFn(item_fn) => self.compile_item_fn(item_fn),
            Item::ItemImport(item_import) => self.compile_item_import(item_import),
        }
    }

    fn compile_item_import(&mut self, import_fn: ItemImport) -> sexpr::Expr {
        let mut imports = vec![];

        for item in import_fn.items {
            let mut import = vec![];

            import.push(s_symbol!("import"));
            import.push(self.compile_ident_string(&import_fn.mod_name));

            match item.kind {
                ImportItemKind::Fn(ImportItemFn {
                    name,
                    params,
                    ret_ty,
                }) => {
                    let mut param_types = vec![];
                    for param in &params {
                        param_types.push(param.ty.clone())
                    }
                    self.scope.add(
                        name.ident.clone(),
                        Ty::Fn {
                            params: param_types,
                            ret: match &ret_ty {
                                Some(ty) => Some(Box::new(ty.clone())),
                                None => None,
                            },
                        },
                    );

                    import.push(self.compile_ident_string(&name));
                    let mut decl = vec![];

                    decl.push(s_symbol!("func"));
                    decl.push(self.compile_ident(&name));

                    for param in params {
                        decl.push(s_list![s_symbol!("param"), self.compile_ty(&param.ty)]);
                    }

                    if matches!(ret_ty, Some(_)) {
                        panic!("cannot specify return type in import function")
                    }

                    import.push(s_list!(decl));
                }
                _ => panic!("unsupported import item"),
            }

            imports.push(s_list!(import));
        }

        s_expand!(imports)
    }

    fn compile_item_fn(&mut self, item_fn: ItemFn) -> sexpr::Expr {
        let mut param_types = vec![];
        for param in &item_fn.params {
            param_types.push(param.ty.clone())
        }
        self.scope.add(
            item_fn.name.ident.clone(),
            Ty::Fn {
                params: param_types,
                ret: match &item_fn.ret_ty {
                    Some(ty) => Some(Box::new(ty.clone())),
                    None => None,
                },
            },
        );
        //let is_ret_void = match &item_fn.ret_ty {
        //    Some(ty) => matches!(ty, Ty::Void),
        //    None => true,
        //};

        self.scope.begin();
        let mut items = vec![];

        items.push(s_symbol!("func"));
        items.push(self.compile_ident(&item_fn.name));

        if item_fn.exported {
            items.push(s_list!(
                s_symbol!("export"),
                self.compile_ident_string(&item_fn.name)
            ));
        }

        for param in item_fn.params {
            items.push(s_list!(vec![
                s_symbol!("param"),
                self.compile_ident(&param.name),
                self.compile_ty(&param.ty),
            ]));
            self.scope.add(param.name.ident, param.ty)
        }

        if let Some(ty) = &item_fn.ret_ty {
            items.push(s_list!(vec![s_symbol!("result"), self.compile_ty(&ty),]));
        }

        for stmt in self.compile_expr_block(&item_fn.body) {
            items.push(stmt);
        }

        if let Some(last_expr) = item_fn.body.last_expr {
            self.last_ret_ty = self.get_type_expr(&last_expr);

            items.push(self.compile_expr(&last_expr));
        }

        if let Some(ty) = &item_fn.ret_ty {
            if &self.last_ret_ty != ty {
                panic!(
                    "expected return type {:?}, got {:?}",
                    item_fn.ret_ty, self.last_ret_ty
                );
            }
        } else {
            if !matches!(&self.last_ret_ty, Ty::Void) {
                panic!("function must not return");
            }
        }

        self.scope.end();
        s_list!(items)
    }

    fn compile_expr_block(&mut self, block: &ExprBlock) -> Vec<sexpr::Expr> {
        let mut items = vec![];

        for stmt in &block.stmts {
            items.push(self.compile_stmt(stmt));
        }

        items
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> sexpr::Expr {
        match &stmt {
            Stmt::StmtSemi(stmt_semi) => self.compile_stmt_semi(stmt_semi),
            Stmt::StmtReturn(stmt_return) => self.compile_stmt_return(stmt_return),
        }
    }

    fn compile_stmt_semi(&mut self, stmt_semi: &StmtSemi) -> sexpr::Expr {
        if matches!(self.get_type_expr(&stmt_semi.expr), Ty::Void) {
            self.compile_expr(&stmt_semi.expr)
        } else {
            s_list!(s_symbol!("drop"), self.compile_expr(&stmt_semi.expr))
        }
    }

    fn compile_stmt_return(&mut self, stmt_return: &StmtReturn) -> sexpr::Expr {
        let mut items = vec![s_symbol!("return")];

        if let Some(expr) = &stmt_return.expr {
            self.last_ret_ty = self.get_type_expr(expr);
            items.push(self.compile_expr(expr));
        } else {
            self.last_ret_ty = Ty::Void
        }

        s_list!(items)
    }

    fn compile_expr(&mut self, expr: &Expr) -> sexpr::Expr {
        match expr {
            Expr::Lit(lit) => self.compile_expr_lit(lit),
            Expr::ExprBinary(expr_binary) => self.compile_expr_binary(expr_binary),
            Expr::ExprCall(expr_call) => self.compile_expr_call(expr_call),
            _ => panic!("unimplemented"),
        }
    }

    fn compile_expr_lit(&mut self, lit: &Lit) -> sexpr::Expr {
        match lit {
            Lit::LitUnsignedInt(lit_unsigned_int) => {
                s_list!(vec![
                    s_symbol!("i32.const"),
                    s_symbol!(lit_unsigned_int.value.to_string()),
                ])
            }
            Lit::LitIdent(lit_ident) => {
                s_list!(vec![s_symbol!("local.get"), self.compile_ident(&lit_ident)])
            }
        }
    }

    fn compile_expr_binary(&mut self, expr_binary: &ExprBinary) -> sexpr::Expr {
        let left_expr = self.compile_expr(&expr_binary.left);
        let left_ty = self.get_type_expr(&expr_binary.left);

        let right_expr = self.compile_expr(&expr_binary.right);
        let right_ty = self.get_type_expr(&expr_binary.right);

        let instruction = get_instruction_binary_op(&expr_binary.op);

        if left_ty != right_ty {
            panic!(
                "unimplemented {:?} {} {:?}",
                &left_ty, instruction, right_ty
            );
        }

        s_list!(vec![
            ty_instruction(&left_ty, &instruction),
            left_expr,
            right_expr,
        ])
    }

    fn compile_expr_call(&mut self, expr_call: &ExprCall) -> sexpr::Expr {
        let name = self.compile_ident(&expr_call.fn_name);
        let name_string = expr_call.fn_name.ident.clone().clone();
        let scope_entity = self.scope.get_with_ref(&name_string);
        if let Some(entity) = scope_entity {
            if let Ty::Fn { params, .. } = &entity.ty {
                let mut items = vec![];

                items.push(s_symbol!("call"));
                items.push(name);

                let params_len = params.len();

                if params_len != expr_call.args.len() {
                    panic!(
                        "[fn {}{}]: expected {} args, got {}",
                        expr_call.fn_name.ident,
                        ty_string(&entity.ty),
                        params_len,
                        expr_call.args.len()
                    );
                }

                for i in 0..params_len {
                    let param = &params[i];
                    let arg = &expr_call.args[i];
                    let arg_ty = self.get_type_expr(arg);

                    if param != &arg_ty {
                        panic!(
                            "[fn {}{}]: expected arg {} to be {:?}, got {:?}",
                            expr_call.fn_name.ident,
                            ty_string(&entity.ty),
                            i,
                            param,
                            arg_ty
                        );
                    }
                }

                for arg in &expr_call.args {
                    items.push(self.compile_expr(arg));
                }

                return s_list!(items);
            } else {
                panic!("expected function type")
            }
        }
        panic!("{} is not defined", expr_call.fn_name.ident);
    }

    fn compile_ty(&mut self, ty: &Ty) -> sexpr::Expr {
        match ty {
            Ty::TyInt64 => s_symbol!("i64"),
            Ty::TyInt32 => s_symbol!("i32"),
            Ty::TyFloat64 => s_symbol!("f64"),
            Ty::TyFloat32 => s_symbol!("f32"),
            _ => panic!("unimplemented"),
        }
    }

    fn compile_ident(&mut self, ident: &LitIdent) -> sexpr::Expr {
        s_symbol!(format!("${}", ident.ident))
    }

    fn compile_ident_string(&mut self, ident: &LitIdent) -> sexpr::Expr {
        s_string!(format!("{}", ident.ident))
    }

    fn get_type_expr(&self, expr: &Expr) -> Ty {
        match expr {
            Expr::Lit(lit) => self.get_type_lit(lit),
            Expr::ExprBinary(expr_binary) => self.get_type_expr_binary(expr_binary),
            Expr::ExprCall(expr_call) => self.get_type_expr_call(expr_call),
            _ => panic!("unimplemented"),
        }
    }

    fn get_type_lit(&self, lit: &Lit) -> Ty {
        match lit {
            Lit::LitUnsignedInt(_) => Ty::TyInt32,
            Lit::LitIdent(lit_ident) => {
                let name = &lit_ident.ident;
                let entity = self.scope.get(name.to_string()).unwrap();

                entity.ty.clone()
            }
        }
    }

    fn get_type_expr_binary(&self, expr_binary: &ExprBinary) -> Ty {
        match expr_binary.op {
            BinaryOp::Add => self.get_type_expr(&expr_binary.left),
            BinaryOp::Sub => self.get_type_expr(&expr_binary.left),
            BinaryOp::Mul => self.get_type_expr(&expr_binary.left),
            BinaryOp::Div => self.get_type_expr(&expr_binary.left),
        }
    }

    fn get_type_expr_call(&self, expr_call: &ExprCall) -> Ty {
        let name = &expr_call.fn_name.ident;
        let entity = self.scope.get(name.to_string()).unwrap();

        if let Ty::Fn { ret, .. } = &entity.ty {
            match &ret {
                Some(ty) => *ty.clone(),
                None => Ty::Void,
            }
        } else {
            panic!("{} is not a function", name);
        }
    }
}

fn get_instruction_binary_op(op: &BinaryOp) -> &str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div",
    }
}

fn ty_instruction(ty: &Ty, instruction: &str) -> sexpr::Expr {
    match ty {
        Ty::TyInt64 => s_symbol!(format!("i64.{}", instruction)),
        Ty::TyInt32 => s_symbol!(format!("i32.{}", instruction)),
        Ty::TyFloat64 => s_symbol!(format!("f64.{}", instruction)),
        Ty::TyFloat32 => s_symbol!(format!("f32.{}", instruction)),
        _ => panic!("unimplemented"),
    }
}

fn ty_string(ty: &Ty) -> String {
    match &ty {
        Ty::TyInt64 => "i64".to_string(),
        Ty::TyInt32 => "i32".to_string(),
        Ty::TyFloat64 => "f64".to_string(),
        Ty::TyFloat32 => "f32".to_string(),
        Ty::Void => "void".to_string(),
        Ty::Fn { params, ret } => {
            format!(
                "({}): {}",
                params.iter().map(ty_string).collect::<Vec<_>>().join(", "),
                match ret {
                    Some(ty) => ty_string(ty),
                    None => ty_string(&Ty::Void),
                }
            )
        }
    }
}
