use crate::{
    ast::{
        lit::lit_ident::LitIdent,
        stmt::{stmt_let::StmtLet, Stmt},
    },
    parser::{
        expr::parse_expr, parser_result::ParseResult, token_walker::TokenWalker, ty::parse_ty,
    },
    tokens::TokenKind,
};

pub fn parse_stmt_let(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    // let <name>: <ty?> = <expr>;
    walker.expect_next_token(TokenKind::KeywordLet)?;
    let name = LitIdent::from_token(walker.next())?;
    let mut ty = None;

    if matches!(walker.peek().kind, TokenKind::Colon) {
        walker.next();
        ty = Some(parse_ty(walker)?);
    }

    walker.expect_next_token(TokenKind::Eq);

    let initializer = parse_expr(walker)?;

    walker.expect_next_token(TokenKind::Semi);

    Ok(Stmt::StmtLet(StmtLet {
        name,
        ty,
        initializer,
    }))
}
