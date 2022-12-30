use crate::{
    ast::expr::{
        expr_unary::{ExprUnary, UnaryOp},
        Expr,
    },
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::expr_prefix_minus::parse_expr_prefix_minus;

pub fn parse_expr_not(walker: &mut TokenWalker) -> ParseResult<Expr> {
    if walker.peek().kind == TokenKind::Exclamation {
        walker.next();
        //let expr = parse_expr_prefix_minus(walker)?;
        let expr = parse_expr_not(walker)?;

        Ok(Expr::ExprUnary(ExprUnary {
            op: UnaryOp::Not,
            expr: Box::new(expr),
        }))
    } else {
        parse_expr_prefix_minus(walker)
    }
}
