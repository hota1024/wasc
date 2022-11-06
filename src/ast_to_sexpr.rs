use crate::{
    ast::{Ast, AstKind, BinOp, Type},
    sexpr::Expr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstToSexprErr {
    UnexpectedAstKind { ast: Ast, expected: String },
    UnknownAstKind { ast: Ast },
}

impl AstToSexprErr {
    pub fn unexpected_ast_kind(ast: Ast, expected: String) -> Self {
        Self::UnexpectedAstKind { ast, expected }
    }

    pub fn unknown_ast_kind(ast: Ast) -> Self {
        Self::UnknownAstKind { ast }
    }
}

pub type AstToSexprResult = Result<Expr, AstToSexprErr>;

pub fn ast_to_sexpr(ast: &Ast) -> AstToSexprResult {
    compile(ast)
}

fn compile(ast: &Ast) -> AstToSexprResult {
    match &ast.kind {
        AstKind::Program { items } => {
            let mut body = Vec::new();

            body.push(Expr::symbol("module".to_string()));

            for item in items {
                body.push(compile(item)?);
            }

            let module = Expr::list(body);
            Ok(module)
        }
        AstKind::ItemGlobal {
            name,
            is_mut,
            ty,
            init,
        } => {
            let mut body = Vec::new();

            body.push(Expr::symbol("global".to_string()));

            body.push(compile(name)?);

            if *is_mut {
                body.push(Expr::list(vec![
                    Expr::symbol("mut".to_string()),
                    compile(ty)?,
                ]));
            } else {
                body.push(compile(ty)?);
            }

            body.push(compile(init)?);

            let global = Expr::list(body);
            Ok(global)
        }
        AstKind::ItemFn {
            exported,
            name,
            params,
            ret_ty,
            body,
        } => {
            let mut func = Vec::new();
            func.push(Expr::symbol("func".to_string()));
            if *exported {
                func.push(Expr::list(vec![
                    Expr::symbol("export".to_string()),
                    ident_to_string(name)?,
                ]));
            }

            for param in params {
                if let AstKind::FnParam { name, ty } = &param.kind {
                    func.push(Expr::list(vec![
                        Expr::symbol("param".to_string()),
                        compile(&name)?,
                        compile(&ty)?,
                    ]))
                } else {
                    panic!("given ast is not a param");
                    todo!("fix this error with Err()!");
                }
            }

            func.push(Expr::list(vec![
                Expr::symbol("result".to_string()),
                compile(ret_ty)?,
            ]));

            let mut func = Expr::list(func);

            if let AstKind::Block { stmts, last_expr } = &body.kind {
                for stmt in stmts {
                    let expr = compile(stmt)?;
                    func.list_append(expr);
                }

                if let Some(last_expr) = last_expr {
                    let expr = compile(last_expr)?;
                    func.list_append(Expr::list_items(vec![
                        expr,
                        Expr::symbol("return".to_string()),
                    ]));
                }
            } else {
                Err(AstToSexprErr::unexpected_ast_kind(
                    *body.clone(),
                    "block".to_string(),
                ))?
            }

            Ok(func)
        }
        AstKind::StmtSemi { expr } => compile(expr),
        AstKind::StmtReturn { expr } => {
            let mut body = Vec::new();
            body.push(compile(expr)?);
            body.push(Expr::symbol("return".to_string()));
            Ok(Expr::list_items(body))
        }
        AstKind::ExprBin { op, left, right } => {
            let mut body = Vec::new();

            body.push(compile(left)?);
            body.push(compile(right)?);

            match op {
                BinOp::Add => body.push(Expr::symbol("i32.add".to_string())),
                BinOp::Sub => body.push(Expr::symbol("i32.sub".to_string())),
                BinOp::Mul => body.push(Expr::symbol("i32.mul".to_string())),
                BinOp::Div => body.push(Expr::symbol("i32.div_s".to_string())),
            }

            let expr = Expr::list_items(body);
            Ok(expr)
        }
        AstKind::Ty { ty } => match ty {
            Type::Int64 => Ok(Expr::symbol("i64".to_string())),
            Type::Int32 => Ok(Expr::symbol("i32".to_string())),
        },
        AstKind::LitIdent(name) => {
            let ident = Expr::symbol(format!("${}", name));
            Ok(ident)
        }
        AstKind::LitUnsignedInt(value) => {
            let value = Expr::symbol(format!("i32.const {}", value));
            Ok(value)
        }
        _ => Err(AstToSexprErr::unknown_ast_kind(ast.clone())),
    }
}

fn ident_to_string(ident: &Ast) -> AstToSexprResult {
    match &ident.kind {
        AstKind::LitIdent(name) => Ok(Expr::string(name.to_string())),
        _ => Err(AstToSexprErr::unexpected_ast_kind(
            ident.clone(),
            "identifier".to_string(),
        )),
    }
}
