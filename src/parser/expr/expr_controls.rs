use crate::{
    ast::expr::{expr_if::ExprIf, expr_while::ExprWhile, Expr},
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::{
    expr_atom::parse_expr_atom,
    expr_block::{parse_block, parse_expr_block},
    parse_expr,
};

pub fn parse_expr_controls(walker: &mut TokenWalker) -> ParseResult<Expr> {
    match walker.peek().kind {
        TokenKind::KeywordIf => parse_expr_if(walker),
        TokenKind::KeywordWhile => parse_expr_while(walker),
        _ => parse_expr_atom(walker),
    }
}

fn parse_expr_if(walker: &mut TokenWalker) -> ParseResult<Expr> {
    walker.expect_next_token(TokenKind::KeywordIf)?;

    let cond = parse_expr(walker)?;
    let then_branch = parse_block(walker)?;
    let else_branch = if walker.peek().kind == TokenKind::KeywordElse {
        walker.next();
        Some(Box::new(match walker.peek().kind {
            TokenKind::OpenBrace => parse_expr_block(walker)?,
            TokenKind::KeywordIf => parse_expr_if(walker)?,
            _ => panic!(
                "else branch should be a block_expr or if_expr, but got `{}`",
                walker.peek().literal
            ),
        }))
    } else {
        None
    };

    Ok(Expr::ExprIf(ExprIf {
        cond: Box::new(cond),
        then_branch,
        else_branch,
    }))
}

fn parse_expr_while(walker: &mut TokenWalker) -> ParseResult<Expr> {
    walker.expect_next_token(TokenKind::KeywordWhile)?;

    let cond = parse_expr(walker)?;
    let body = parse_block(walker)?;

    Ok(Expr::ExprWhile(ExprWhile {
        cond: Box::new(cond),
        body,
    }))
}
