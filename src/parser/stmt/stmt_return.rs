use crate::{
    ast::stmt::{Stmt, StmtReturn::StmtReturn},
    parser::{expr::parse_expr, parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

pub fn parse_return(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    walker.expect_next_token(TokenKind::KeywordReturn)?;

    if walker.peek().kind == TokenKind::Semi {
        walker.next();
        return Ok(Stmt::StmtReturn(StmtReturn { expr: None }));
    }

    let expr = parse_expr(walker)?;
    walker.expect_next_token(TokenKind::Semi)?;
    Ok(Stmt::StmtReturn(StmtReturn { expr: Some(expr) }))
}
