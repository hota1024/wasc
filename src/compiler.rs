use crate::ast::{Ast, AstKind};
use crate::sexpr::Expr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompileErr {
    UnexpectedAstKind { ast: Ast, expected: String },
    UnknownAstKind { ast: Ast },
}

impl CompileErr {
    pub fn unexpected_ast_kind(ast: Ast, expected: String) -> Self {
        Self::UnexpectedAstKind { ast, expected }
    }

    pub fn unknown_ast_kind(ast: Ast) -> Self {
        Self::UnknownAstKind { ast }
    }
}

pub type CompileResult = Result<Expr, CompileErr>;

pub struct Context {}

pub struct Compiler {
    context: Context,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            context: Context {},
        }
    }

    pub fn compile(&mut self, ast: Ast) -> CompileResult {
        match &ast.kind {
            AstKind::Program { .. } => self.compile_program(ast),
            _ => Err(CompileErr::unknown_ast_kind(ast.clone())),
        }
    }

    pub fn compile_program(&mut self, ast: Ast) -> CompileResult {
        if let AstKind::Program { items } = ast.kind {
            let mut body = vec![];

            body.push(Expr::symbol("module".to_string()));

            for item in items {
                body.push(self.compile(item)?);
            }

            let module = Expr::list(body);
            Ok(module)
        } else {
            Err(CompileErr::unexpected_ast_kind(
                ast.clone(),
                "program".to_string(),
            ))
        }
    }
}
