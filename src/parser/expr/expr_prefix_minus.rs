use crate::{
    ast::expr::{
        expr_unary::{ExprUnary, UnaryOp},
        Expr,
    },
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::expr_as::parse_expr_as;

pub fn parse_expr_prefix_minus(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let peek = walker.peek();

    match peek.kind {
        TokenKind::Minus => {
            walker.next();
            let expr = parse_expr_as(walker)?;

            Ok(Expr::ExprUnary(ExprUnary {
                op: UnaryOp::Minus,
                expr: Box::new(expr),
            }))
        }
        _ => parse_expr_as(walker),
    }
}
