use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_equality::parse_expr_equality, parse_binary::parse_binary_expr};

pub fn parse_expr_and(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_equality, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::AndAnd => {
                walker.next();
                Ok(BinaryOp::And)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::AndAnd],
            }),
        }
    })
}
