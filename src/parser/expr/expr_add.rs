use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_mul::parse_expr_mul, parse_binary::parse_binary_expr};

pub fn parse_expr_add(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_mul, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::Plus => {
                walker.next();
                Ok(BinaryOp::Add)
            }
            TokenKind::Minus => {
                walker.next();
                Ok(BinaryOp::Sub)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::Star, TokenKind::Slash],
            }),
        }
    })
}
