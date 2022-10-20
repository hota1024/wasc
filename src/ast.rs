use crate::{
    parse_err::ParseErr,
    span::Span,
    tokens::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int64,
    Int32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Program {
        items: Vec<Ast>,
    },
    Ty {
        ty: Type,
    },
    ItemGlobal {
        name: Box<Ast>,
        is_mut: bool,
        ty: Box<Ast>,
        init: Box<Ast>,
    },
    ItemFn {
        name: Box<Ast>,
        ret_ty: Box<Ast>,
        body: Box<Ast>,
    },
    Block {
        stmts: Vec<Ast>,
    },
    StmtSemi {
        expr: Box<Ast>,
    },
    StmtReturn {
        expr: Box<Ast>,
    },
    ExprBin {
        op: BinOp,
        left: Box<Ast>,
        right: Box<Ast>,
    },
    LitUnsignedInt(u64),
    LitIdent(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ast {
    pub kind: AstKind,
    pub span: Span,
}

impl Ast {
    pub fn new(kind: AstKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn program(stmts: Vec<Ast>) -> Self {
        let span = if let Some(first) = stmts.first() {
            first.span.merge(&stmts.last().unwrap().span)
        } else {
            Span::new(0, 0)
        };

        Self::new(
            AstKind::Program {
                items: stmts.to_vec(),
            },
            span,
        )
    }

    pub fn item_let(
        global_token: &Token,
        name: Ast,
        is_mut: bool,
        ty: Ast,
        init: Ast,
        semi_token: &Token,
    ) -> Self {
        let span = global_token.span.merge(&semi_token.span);

        Self::new(
            AstKind::ItemGlobal {
                name: Box::new(name),
                is_mut,
                ty: Box::new(ty),
                init: Box::new(init),
            },
            span,
        )
    }

    pub fn item_fn(fn_token: &Token, name: Ast, ret_type: Ast, block: Ast) -> Self {
        let span = fn_token.span.merge(&block.span);

        Self::new(
            AstKind::ItemFn {
                name: Box::new(name),
                ret_ty: Box::new(ret_type),
                body: Box::new(block),
            },
            span,
        )
    }

    pub fn block(open_token: &Token, stmts: Vec<Ast>, close_token: &Token) -> Self {
        let span = open_token.span.merge(&close_token.span);

        Self::new(AstKind::Block { stmts }, span)
    }

    pub fn stmt_semi(expr: Ast, semi_token: &Token) -> Self {
        let span = expr.span.merge(&semi_token.span);

        Self::new(
            AstKind::StmtSemi {
                expr: Box::new(expr),
            },
            span,
        )
    }

    pub fn stmt_return(return_token: &Token, expr: Ast, semi_token: &Token) -> Self {
        let span = return_token.span.merge(&semi_token.span);

        Self::new(
            AstKind::StmtReturn {
                expr: Box::new(expr),
            },
            span,
        )
    }

    pub fn expr_bin(op: BinOp, left: Ast, right: Ast) -> Self {
        let span = left.span.merge(&right.span);
        Self::new(
            AstKind::ExprBin {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            span,
        )
    }

    pub fn ident_from_token(token: &Token) -> Result<Self, ParseErr> {
        match &token.kind {
            TokenKind::Ident(name) => {
                Ok(Self::new(AstKind::LitIdent(name.to_string()), token.span))
            }
            _ => Err(ParseErr::unexpected_token(token.clone(), format!("ident"))),
        }
    }

    pub fn lit_from_token(token: &Token) -> Result<Self, ParseErr> {
        match token {
            Token {
                kind: TokenKind::UnsignedInt(i),
                ..
            } => Ok(Self::new(AstKind::LitUnsignedInt(i.clone()), token.span)),
            _ => Err(ParseErr::unexpected_token(
                token.clone(),
                "integer".to_string(),
            )),
        }
    }

    pub fn ty_from_token(token: &Token) -> Result<Self, ParseErr> {
        match token {
            Token {
                kind: TokenKind::KeywordI64,
                ..
            } => Ok(Self::new(AstKind::Ty { ty: Type::Int64 }, token.span)),
            Token {
                kind: TokenKind::KeywordI32,
                ..
            } => Ok(Self::new(AstKind::Ty { ty: Type::Int32 }, token.span)),
            _ => Err(ParseErr::unexpected_token(token.clone(), format!("i64"))),
        }
    }
}
