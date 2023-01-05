use crate::{
    ast::expr::{expr_as::ExprAs, Expr},
    parser::{parser_result::ParseResult, token_walker::TokenWalker, ty::parse_ty},
    tokens::TokenKind,
};

use super::expr_controls::parse_expr_controls;

pub fn parse_expr_as(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let mut expr = parse_expr_controls(walker)?;

    loop {
        if walker.peek().kind == TokenKind::KeywordAs {
            walker.next();
            let ty = parse_ty(walker)?;

            expr = Expr::ExprAs(ExprAs {
                expr: Box::new(expr),
                ty,
            });
        } else {
            break;
        }
    }

    Ok(expr)
}
