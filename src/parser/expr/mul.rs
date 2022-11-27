use crate::{
    ast::expr::{Expr, ExprBinary::BinaryOp},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{parse_binary::parse_binary_expr, unary::parse_unary};

pub fn parse_mul(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_unary, |walker| {
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
