use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_unary::parse_expr_unary, parse_binary::parse_binary_expr};

pub fn parse_expr_mul(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_unary, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::Star => {
                walker.next();
                Ok(BinaryOp::Mul)
            }
            TokenKind::Slash => {
                walker.next();
                Ok(BinaryOp::Div)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::Star, TokenKind::Slash],
            }),
        }
    })
}
