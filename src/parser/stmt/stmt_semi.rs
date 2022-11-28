use crate::{
    ast::stmt::{stmt_semi::StmtSemi, Stmt},
    parser::{expr::parse_expr, parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

pub fn parse_stmt_semi(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    let expr = parse_expr(walker)?;
    walker.expect_next_token(TokenKind::Semi)?;

    Ok(Stmt::StmtSemi(StmtSemi {
        expr: Box::new(expr),
    }))
}
