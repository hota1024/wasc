use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_add::parse_expr_add, parse_binary::parse_binary_expr};

pub fn parse_expr_relational(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_add, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::Gt => {
                walker.next();
                Ok(BinaryOp::Gt)
            }
            TokenKind::Lt => {
                walker.next();
                Ok(BinaryOp::Lt)
            }
            TokenKind::Ge => {
                walker.next();
                Ok(BinaryOp::Ge)
            }
            TokenKind::Le => {
                walker.next();
                Ok(BinaryOp::Le)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::Gt, TokenKind::Lt, TokenKind::Ge, TokenKind::Le],
            }),
        }
    })
}
