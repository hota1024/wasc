use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_and::parse_expr_and, parse_binary::parse_binary_expr};

pub fn parse_expr_or(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_and, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::BarBar => {
                walker.next();
                Ok(BinaryOp::Or)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::BarBar],
            }),
        }
    })
}
