pub mod scope;

use std::fmt::Binary;

use crate::{
    ast::{
        expr::{
            expr_binary::{BinaryOp, ExprBinary},
            expr_block::ExprBlock,
            Expr,
        },
        item::{item_fn::ItemFn, Item},
        lit::{lit_ident::LitIdent, Lit},
        module::Module,
        stmt::{stmt_return::StmtReturn, stmt_semi::StmtSemi, Stmt},
        ty::Ty,
    },
    sexpr::{self, s_list, s_string, s_symbol},
};

use self::scope::Scope;

pub struct Compiler {
    scope: Scope,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
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
        }
    }

    fn compile_item_fn(&mut self, item_fn: ItemFn) -> sexpr::Expr {
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
        }

        items.push(s_list!(vec![
            s_symbol!("result"),
            self.compile_ty(&item_fn.ret_ty),
        ]));

        for stmt in self.compile_expr_block(&item_fn.body) {
            items.push(stmt);
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
        self.compile_expr(&stmt_semi.expr)
    }

    fn compile_stmt_return(&mut self, stmt_return: &StmtReturn) -> sexpr::Expr {
        let mut items = vec![s_symbol!("return")];

        if let Some(expr) = &stmt_return.expr {
            items.push(self.compile_expr(expr));
        }

        s_list!(items)
    }

    fn compile_expr(&mut self, expr: &Expr) -> sexpr::Expr {
        match expr {
            Expr::Lit(lit) => self.compile_expr_lit(lit),
            Expr::ExprBinary(expr_binary) => self.compile_expr_binary(expr_binary),
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
            _ => panic!("unimplemented"),
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

    fn compile_ty(&mut self, ty: &Ty) -> sexpr::Expr {
        match ty {
            Ty::TyInt64 => s_symbol!("i64"),
            Ty::TyInt32 => s_symbol!("i32"),
            Ty::TyFloat64 => s_symbol!("f64"),
            Ty::TyFloat32 => s_symbol!("f32"),
        }
    }

    fn compile_ident(&mut self, ident: &LitIdent) -> sexpr::Expr {
        s_symbol!(format!("${}", ident.ident))
    }

    fn compile_ident_string(&mut self, ident: &LitIdent) -> sexpr::Expr {
        s_string!(format!("{}", ident.ident))
    }

    fn get_type_expr(&mut self, expr: &Expr) -> Ty {
        match expr {
            Expr::Lit(lit) => self.get_type_lit(lit),
            Expr::ExprBinary(expr_binary) => self.get_type_expr_binary(expr_binary),
            _ => panic!("unimplemented"),
        }
    }

    fn get_type_lit(&mut self, lit: &Lit) -> Ty {
        match lit {
            Lit::LitUnsignedInt(_) => Ty::TyInt32,
            _ => panic!("unimplemented"),
        }
    }

    fn get_type_expr_binary(&mut self, expr_binary: &ExprBinary) -> Ty {
        match expr_binary.op {
            BinaryOp::Add => self.get_type_expr(&expr_binary.left),
            BinaryOp::Sub => self.get_type_expr(&expr_binary.left),
            BinaryOp::Mul => self.get_type_expr(&expr_binary.left),
            BinaryOp::Div => self.get_type_expr(&expr_binary.left),
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
    }
}
