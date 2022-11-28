use crate::{
    ast::stmt::{stmt_return::StmtReturn, Stmt},
    parser::{expr::parse_expr, parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

pub fn parse_stmt_return(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    walker.expect_next_token(TokenKind::KeywordReturn)?;

    if walker.peek().kind == TokenKind::Semi {
        walker.next();
        return Ok(Stmt::StmtReturn(StmtReturn { expr: None }));
    }

    let expr = parse_expr(walker)?;
    walker.expect_next_token(TokenKind::Semi)?;
    Ok(Stmt::StmtReturn(StmtReturn { expr: Some(expr) }))
}
