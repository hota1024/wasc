use crate::{
    ast::expr::{
        Expr,
        ExprUnary::{ExprUnary, UnaryOp},
    },
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::atom::parse_atom;

pub fn parse_unary(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let peek = walker.peek();

    match peek.kind {
        TokenKind::Minus => {
            walker.next();
            let expr = parse_atom(walker)?;

            Ok(Expr::ExprUnary(ExprUnary {
                op: UnaryOp::Minus,
                expr: Box::new(expr),
            }))
        }
        _ => parse_atom(walker),
    }
}
