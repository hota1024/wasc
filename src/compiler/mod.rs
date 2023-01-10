pub mod scope;

use crate::{
    ast::{
        expr::{
            expr_as::ExprAs,
            expr_binary::{AssignOp, BinaryOp, ExprBinary},
            expr_block::ExprBlock,
            expr_call::ExprCall,
            expr_if::ExprIf,
            expr_unary::{ExprUnary, UnaryOp},
            Expr,
        },
        item::{
            item_fn::ItemFn,
            item_import::{ImportItemFn, ImportItemKind, ItemImport},
            Item,
        },
        lit::{lit_ident::LitIdent, Lit},
        module::Module,
        stmt::{stmt_let::StmtLet, stmt_return::StmtReturn, stmt_semi::StmtSemi, Stmt},
        ty::{ty_string, Ty},
    },
    sexpr::{self, s_expand, s_list, s_string, s_symbol},
    wasm::WasmTy,
};

use self::scope::Scope;

struct LastExprGlobal {
    global_prefix: &'static str,
    ty: Option<Ty>,
}

impl LastExprGlobal {
    pub fn new(global_prefix: &'static str) -> Self {
        LastExprGlobal {
            global_prefix,
            ty: None,
        }
    }

    pub fn declare(&self, ty: &WasmTy, init: sexpr::Expr) -> sexpr::Expr {
        s_list!(
            s_symbol!("global"),
            self.to_name_symbol_as(ty),
            s_list!(s_symbol!("mut"), s_symbol!(ty)),
            init
        )
    }

    pub fn to_name_symbol_as(&self, ty: &WasmTy) -> sexpr::Expr {
        s_symbol!(format!("${}{}", self.global_prefix, ty))
    }

    pub fn acquire(&mut self, ty: Ty) {
        if let Some(ty) = &self.ty {
            panic!("this global is already acquired")
        }

        self.ty = Some(ty);
    }

    pub fn free(&mut self) {
        if self.ty.is_none() {
            panic!("this global is not acquired")
        }
    }
}

pub struct Compiler {
    scope: Scope,
    last_ret_ty: Ty,
    expect_lit_ty: Option<Ty>,
    last_expr_global: LastExprGlobal,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            last_ret_ty: Ty::Void,
            expect_lit_ty: None,
            last_expr_global: LastExprGlobal::new("__wasm_block_"),
        }
    }

    pub fn compile_module(&mut self, module: Module) -> sexpr::Expr {
        let mut items = vec![];
        items.push(s_symbol!("module"));

        let mut last_item = None;

        for item in module.items {
            if let Some(last_item) = last_item {
                if let Item::ItemImport(_) = last_item {
                    // add global_if_result
                    items.push(self.last_expr_global.declare(
                        &WasmTy::Int32,
                        s_list!(s_symbol!("i32.const"), s_symbol!("0")),
                    ));
                    items.push(self.last_expr_global.declare(
                        &WasmTy::Int64,
                        s_list!(s_symbol!("i64.const"), s_symbol!("0")),
                    ));
                    items.push(self.last_expr_global.declare(
                        &WasmTy::Float32,
                        s_list!(s_symbol!("f32.const"), s_symbol!("0")),
                    ));
                    items.push(self.last_expr_global.declare(
                        &WasmTy::Float64,
                        s_list!(s_symbol!("f64.const"), s_symbol!("0")),
                    ));
                }
            }

            items.push(self.compile_item(&item));
            last_item = Some(item.clone());
        }

        s_list!(items)
    }

    fn compile_item(&mut self, item: &Item) -> sexpr::Expr {
        match item {
            Item::ItemFn(item_fn) => self.compile_item_fn(&item_fn),
            Item::ItemImport(item_import) => self.compile_item_import(&item_import),
        }
    }

    fn compile_item_import(&mut self, import_fn: &ItemImport) -> sexpr::Expr {
        let mut imports = vec![];

        for item in &import_fn.items {
            let mut import = vec![];

            import.push(s_symbol!("import"));
            import.push(self.compile_ident_string(&import_fn.mod_name));

            match &item.kind {
                ImportItemKind::Fn(ImportItemFn {
                    name,
                    params,
                    ret_ty,
                }) => {
                    let mut param_types = vec![];
                    for param in params {
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

    fn compile_item_fn(&mut self, item_fn: &ItemFn) -> sexpr::Expr {
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

        for param in &item_fn.params {
            items.push(s_list!(vec![
                s_symbol!("param"),
                self.compile_ident(&param.name),
                self.compile_ty(&param.ty),
            ]));
            self.scope.add(param.name.ident.clone(), param.ty.clone())
        }

        for local in self.compile_let_decl_in_block(&item_fn.body) {
            items.push(local);
        }

        if let Some(ty) = &item_fn.ret_ty {
            items.push(s_list!(vec![s_symbol!("result"), self.compile_ty(&ty),]));
        }

        for stmt in self.compile_expr_block(&item_fn.body) {
            items.push(stmt);
        }

        if let Some(last_expr) = &item_fn.body.last_expr {
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

    fn compile_let_decl_in_block(&mut self, block: &ExprBlock) -> Vec<sexpr::Expr> {
        let mut locals = vec![];

        for stmt in &block.stmts {
            match stmt {
                Stmt::StmtLet(StmtLet {
                    name,
                    ty,
                    initializer,
                }) => {
                    let ty = match &ty {
                        Some(ty) => ty.clone(),
                        None => self.get_type_expr(&initializer),
                    };
                    locals.push(s_list!(
                        s_symbol!("local"),
                        self.compile_ident(&name),
                        self.compile_ty(&ty)
                    ));
                }
                _ => (),
            }
        }

        locals
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
            Stmt::StmtLet(stmt_let) => self.compile_stmt_let(stmt_let),
        }
    }

    fn compile_stmt_let(&mut self, stmt_let: &StmtLet) -> sexpr::Expr {
        self.expect_lit_ty = stmt_let.ty.clone();
        let initializer_ty = self.get_type_expr(&stmt_let.initializer);
        self.expect_lit_ty = None;

        if let Some(ty) = &stmt_let.ty {
            if &initializer_ty != ty {
                panic!(
                    "cannot initialize {} by {}",
                    stmt_let.name.ident,
                    ty_string(&initializer_ty)
                )
            }
        }

        let mut initializer_items = vec![];

        initializer_items.push(s_symbol!("local.set"));
        initializer_items.push(self.compile_ident(&stmt_let.name));

        self.expect_lit_ty = stmt_let.ty.clone();
        initializer_items.push(self.compile_expr(&stmt_let.initializer));
        self.expect_lit_ty = None;

        self.scope.add(stmt_let.name.ident.clone(), initializer_ty);
        s_list!(initializer_items)
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
            Expr::ExprUnary(expr_unary) => self.compile_expr_unary(expr_unary),
            Expr::ExprCall(expr_call) => self.compile_expr_call(expr_call),
            Expr::ExprAs(expr_as) => self.compile_expr_as(expr_as),
            Expr::ExprIf(expr_if) => self.compile_expr_if(expr_if),
            _ => panic!("unimplemented expression compiler for `{:?}`", expr),
        }
    }

    fn compile_expr_lit(&mut self, lit: &Lit) -> sexpr::Expr {
        match lit {
            Lit::LitUnsignedInt(lit_unsigned_int) => {
                s_list!(vec![
                    if let Some(ty) = &self.expect_lit_ty {
                        match ty {
                            Ty::TyInt32 => s_symbol!("i32.const"),
                            Ty::TyInt64 => s_symbol!("i64.const"),
                            Ty::TyFloat32 => s_symbol!("f32.const"),
                            Ty::TyFloat64 => s_symbol!("f64.const"),
                            _ => panic!("cannot use {} for integer literal", ty_string(&ty)),
                        }
                    } else {
                        s_symbol!("i32.const")
                    },
                    s_symbol!(lit_unsigned_int.value.to_string()),
                ])
            }
            Lit::LitUnsignedFloat(lit_unsigned_int) => {
                s_list!(vec![
                    if let Some(ty) = &self.expect_lit_ty {
                        match ty {
                            Ty::TyFloat32 => s_symbol!("f32.const"),
                            Ty::TyFloat64 => s_symbol!("f64.const"),
                            _ => panic!("cannot use {} for float literal", ty_string(&ty)),
                        }
                    } else {
                        s_symbol!("f32.const")
                    },
                    s_symbol!(lit_unsigned_int.value.to_string()),
                ])
            }
            Lit::LitIdent(lit_ident) => {
                s_list!(vec![s_symbol!("local.get"), self.compile_ident(&lit_ident)])
            }
            Lit::LitBool(lit_bool) => {
                if lit_bool.value {
                    s_list!(s_symbol!("i32.const"), s_symbol!("1"))
                } else {
                    s_list!(s_symbol!("i32.const"), s_symbol!("0"))
                }
            }
        }
    }

    fn compile_expr_binary(&mut self, expr_binary: &ExprBinary) -> sexpr::Expr {
        match &expr_binary.op {
            BinaryOp::Assign => {
                if let Expr::Lit(Lit::LitIdent(ident)) = &*expr_binary.left {
                    let expr = self.compile_expr(&expr_binary.right);
                    let expr_ty = self.get_type_expr(&expr_binary.right);
                    let variable = self
                        .scope
                        .get(ident.ident.clone())
                        .expect(format!("undefined variable: `{}`", ident.ident).as_str());

                    if variable.ty != expr_ty {
                        panic!(
                            "cannnot assign: {}: {} = <{}>",
                            ident.ident,
                            ty_string(&variable.ty),
                            ty_string(&expr_ty)
                        );
                    }

                    s_list!(s_symbol!("local.set"), self.compile_ident(&ident), expr)
                } else {
                    panic!("left hand must be identifier in assign expression")
                }
            }
            BinaryOp::AssignOp(op) => {
                if let Expr::Lit(Lit::LitIdent(ident)) = &*expr_binary.left {
                    let mut expr = self.compile_expr(&expr_binary.right);
                    let expr_ty = self.get_type_expr(&expr_binary.right);
                    let variable = self
                        .scope
                        .get(ident.ident.clone())
                        .expect(format!("undefined variable: `{}`", ident.ident).as_str());

                    if variable.ty != expr_ty {
                        panic!(
                            "cannnot assign: {}: {} = <{}>",
                            ident.ident,
                            ty_string(&variable.ty),
                            ty_string(&expr_ty)
                        );
                    }

                    let instruction = match op {
                        AssignOp::Add => get_instruction_binary_op(&BinaryOp::Add),
                        AssignOp::Sub => get_instruction_binary_op(&BinaryOp::Sub),
                        AssignOp::Mul => get_instruction_binary_op(&BinaryOp::Mul),
                        AssignOp::Div => get_instruction_binary_op(&BinaryOp::Div),
                    };

                    expr = s_list!(vec![
                        ty_instruction(&variable.ty, &instruction),
                        s_list!(s_symbol!("local.get"), self.compile_ident(ident)),
                        expr,
                    ]);

                    s_list!(s_symbol!("local.set"), self.compile_ident(&ident), expr)
                } else {
                    panic!("left hand must be identifier in assign expression")
                }
            }
            _ => {
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
        }
    }

    fn compile_expr_unary(&mut self, expr_unary: &ExprUnary) -> sexpr::Expr {
        match &expr_unary.op {
            UnaryOp::Not => {
                s_list!(vec![
                    s_symbol!("i32.eqz"),
                    self.compile_expr(&expr_unary.expr)
                ])
            }
            _ => panic!("unimplemented unary operator `{:?}`", expr_unary.op),
        }
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

    fn compile_expr_as(&mut self, expr_as: &ExprAs) -> sexpr::Expr {
        let expr = self.compile_expr(&expr_as.expr);
        let expr_ty = self.get_type_expr(&expr_as.expr);

        match expr_ty {
            Ty::TyInt32 => match expr_as.ty {
                // i32 to i32
                Ty::TyInt32 => expr,
                // i32 to i64
                Ty::TyInt64 => s_list!(s_symbol!("i64.extend_i32_s"), expr),
                // i32 to f32
                Ty::TyFloat32 => s_list!(s_symbol!("f32.convert_i32_s"), expr),
                // i32 to f64
                Ty::TyFloat64 => s_list!(s_symbol!("f64.convert_i32_s"), expr),
                _ => panic!(
                    "unsportted type casting to {} from {}",
                    ty_string(&expr_as.ty),
                    ty_string(&expr_ty)
                ),
            },
            Ty::TyInt64 => match expr_as.ty {
                // i64 to i32
                Ty::TyInt32 => s_list!(s_symbol!("i32.wrap_i64"), expr),
                // i64 to i64
                Ty::TyInt64 => expr,
                // i64 to f32
                Ty::TyFloat32 => s_list!(s_symbol!("f32.convert_i64_s"), expr),
                // i64 to f64
                Ty::TyFloat64 => s_list!(s_symbol!("f64.convert_i64_s"), expr),
                _ => panic!(
                    "unsportted type casting to {} from {}",
                    ty_string(&expr_as.ty),
                    ty_string(&expr_ty)
                ),
            },
            Ty::TyFloat32 => match expr_as.ty {
                // f32 to i32
                Ty::TyInt32 => s_list!(s_symbol!("i32.trunc_f32_s"), expr),
                // f32 to i64
                Ty::TyInt64 => s_list!(s_symbol!("i64.trunc_f32_s"), expr),
                // f32 to f32
                Ty::TyFloat32 => expr,
                // f32 to f64
                Ty::TyFloat64 => s_list!(s_symbol!("f64.promote_f32"), expr),
                _ => panic!(
                    "unsportted type casting to {} from {}",
                    ty_string(&expr_as.ty),
                    ty_string(&expr_ty)
                ),
            },
            Ty::TyFloat64 => match expr_as.ty {
                // f64 to i32
                Ty::TyInt32 => s_list!(s_symbol!("i32.trunc_f64_s"), expr),
                // f64 to i64
                Ty::TyInt64 => s_list!(s_symbol!("i64.trunc_f64_s"), expr),
                // f64 to f32
                Ty::TyFloat32 => s_list!(s_symbol!("f32.demote_f64"), expr),
                // f64 to f64
                Ty::TyFloat64 => expr,
                _ => panic!(
                    "unsportted type casting to {} from {}",
                    ty_string(&expr_as.ty),
                    ty_string(&expr_ty)
                ),
            },
            _ => panic!("unsportted type casting source: {}", ty_string(&expr_ty)),
        }
    }

    fn compile_expr_if(&mut self, expr_if: &ExprIf) -> sexpr::Expr {
        let mut expand_items = vec![];
        expand_items.push(self.compile_expr(&expr_if.cond));

        let mut if_items = vec![];
        if_items.push(s_symbol!("if"));

        let mut then_items = vec![s_symbol!("then")];
        for stmt in self.compile_expr_block(&expr_if.then_branch) {
            then_items.push(stmt);
        }
        if_items.push(s_list!(then_items));

        // then-branch return
        if let Some(last_expr) = &expr_if.then_branch.last_expr {
            let last_expr_ty = self.get_type_expr(last_expr);
            if last_expr_ty != Ty::Void {}
        }

        if let Some(else_branch) = &expr_if.else_branch {
            let mut else_items = vec![s_symbol!("else")];

            match else_branch.as_ref() {
                Expr::ExprBlock(else_block) => {
                    for stmt in self.compile_expr_block(&else_block) {
                        else_items.push(stmt);
                    }
                }
                Expr::ExprIf(expr_if) => {
                    else_items.push(self.compile_expr_if(&expr_if));
                }
                _ => panic!("else branch should be a block_expr or if_expr node"),
            }

            if_items.push(s_list!(else_items));
        };

        expand_items.push(s_list!(if_items));

        s_expand!(expand_items)
    }

    fn compile_ty(&mut self, ty: &Ty) -> sexpr::Expr {
        match ty {
            Ty::TyInt64 => s_symbol!("i64"),
            Ty::TyInt32 => s_symbol!("i32"),
            Ty::TyFloat64 => s_symbol!("f64"),
            Ty::TyFloat32 => s_symbol!("f32"),
            Ty::TyBool => s_symbol!("i32"),
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
            Expr::ExprUnary(expr_unary) => self.get_type_expr_unary(expr_unary),
            Expr::ExprCall(expr_call) => self.get_type_expr_call(expr_call),
            Expr::ExprAs(expr_as) => self.get_type_expr_as(expr_as),
            Expr::ExprBlock(expr_block) => self.get_type_expr_block(expr_block),
            Expr::ExprIf(expr_if) => self.get_type_expr_if(expr_if),
            _ => panic!("unimplemented type getter for `{:?}`", expr),
        }
    }

    fn get_type_lit(&self, lit: &Lit) -> Ty {
        match lit {
            Lit::LitUnsignedInt(_) => {
                if let Some(ty) = &self.expect_lit_ty {
                    ty.clone()
                } else {
                    Ty::TyInt32
                }
            }
            Lit::LitUnsignedFloat(_) => {
                if let Some(ty) = &self.expect_lit_ty {
                    ty.clone()
                } else {
                    Ty::TyFloat32
                }
            }
            Lit::LitIdent(lit_ident) => {
                let name = &lit_ident.ident;

                let entity = self
                    .scope
                    .get(name.to_string())
                    .expect(format!("`{}` is not defined", name).as_str());

                entity.ty.clone()
            }
            Lit::LitBool(_) => Ty::TyBool,
        }
    }

    fn get_type_expr_binary(&self, expr_binary: &ExprBinary) -> Ty {
        match expr_binary.op {
            BinaryOp::Add => self.get_type_expr(&expr_binary.left),
            BinaryOp::Sub => self.get_type_expr(&expr_binary.left),
            BinaryOp::Mul => self.get_type_expr(&expr_binary.left),
            BinaryOp::Div => self.get_type_expr(&expr_binary.left),
            BinaryOp::Assign | BinaryOp::AssignOp(..) => Ty::Void,
            BinaryOp::Lt => Ty::TyBool,
            BinaryOp::Gt => Ty::TyBool,
            BinaryOp::Le => Ty::TyBool,
            BinaryOp::Ge => Ty::TyBool,
            BinaryOp::EqEq => Ty::TyBool,
            BinaryOp::NotEq => Ty::TyBool,
            BinaryOp::And => Ty::TyBool,
            BinaryOp::Or => Ty::TyBool,
        }
    }

    fn get_type_expr_unary(&self, expr_unary: &ExprUnary) -> Ty {
        match expr_unary.op {
            UnaryOp::Not => Ty::TyBool,
            _ => panic!("unimplemented unary operator `{:?}`", expr_unary.op),
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

    fn get_type_expr_as(&self, expr_as: &ExprAs) -> Ty {
        expr_as.ty.clone()
    }

    fn get_type_expr_block(&self, expr_block: &ExprBlock) -> Ty {
        if let Some(last_expr) = &expr_block.last_expr {
            self.get_type_expr(&last_expr)
        } else {
            Ty::Void
        }
    }

    fn get_type_expr_if(&self, expr_if: &ExprIf) -> Ty {
        self.get_type_expr(&Expr::ExprBlock(expr_if.then_branch.clone()))
    }
}

fn get_instruction_binary_op(op: &BinaryOp) -> &str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Sub => "sub",
        BinaryOp::Mul => "mul",
        BinaryOp::Div => "div_s",
        BinaryOp::Lt => "lt_s",
        BinaryOp::Gt => "gt_s",
        BinaryOp::Le => "le_s",
        BinaryOp::Ge => "ge_s",
        BinaryOp::EqEq => "eq",
        BinaryOp::NotEq => "ne",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        _ => panic!("no instruction for binary operator `{:?}`", op),
    }
}

fn ty_instruction(ty: &Ty, instruction: &str) -> sexpr::Expr {
    match ty {
        Ty::TyInt64 => s_symbol!(format!("i64.{}", instruction)),
        Ty::TyInt32 => s_symbol!(format!("i32.{}", instruction)),
        Ty::TyFloat64 => s_symbol!(format!("f64.{}", instruction)),
        Ty::TyFloat32 => s_symbol!(format!("f32.{}", instruction)),
        Ty::TyBool => s_symbol!(format!("i32.{}", instruction)),
        _ => panic!("unimplemented type instruction: `{}`", ty_string(&ty)),
    }
}
