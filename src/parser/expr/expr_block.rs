use crate::{
    ast::expr::{expr_block::ExprBlock, Expr},
    parser::{parser_result::ParseResult, stmt::parse_stmt, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::parse_expr;

pub fn parse_expr_block(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let block = parse_block(walker)?;

    Ok(Expr::ExprBlock(block))
    // let mut stmts = Vec::new();
    // let mut last_expr = None;
    // walker.expect_next_token(TokenKind::OpenBrace)?;

    // while walker.peek().kind != TokenKind::CloseBrace {
    //     let pos = walker.get_pos();
    //     if let Ok(stmt) = parse_stmt(walker) {
    //         stmts.push(stmt);
    //     } else {
    //         walker.set_pos(pos);
    //         last_expr = Some(Box::new(parse_expr(walker)?));
    //         walker.expect_next_token(TokenKind::CloseBrace)?;
    //         break;
    //     }
    // }

    // Ok(Expr::ExprBlock(ExprBlock { stmts, last_expr }))
}

pub fn parse_block(walker: &mut TokenWalker) -> ParseResult<ExprBlock> {
    let mut stmts = Vec::new();
    let mut last_expr = None;
    walker.expect_next_token(TokenKind::OpenBrace)?;

    while walker.peek().kind != TokenKind::CloseBrace {
        let pos = walker.get_pos();
        if let Ok(stmt) = parse_stmt(walker) {
            stmts.push(stmt);
        } else {
            walker.set_pos(pos);
            last_expr = Some(Box::new(parse_expr(walker)?));
            break;
        }
    }

    walker.expect_next_token(TokenKind::CloseBrace)?;

    Ok(ExprBlock { stmts, last_expr })
}
