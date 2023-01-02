use crate::{
    ast::expr::{expr_binary::BinaryOp, Expr},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_relational::parse_expr_relational, parse_binary::parse_binary_expr};

pub fn parse_expr_equality(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_relational, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::EqEq => {
                walker.next();
                Ok(BinaryOp::EqEq)
            }
            TokenKind::NotEq => {
                walker.next();
                Ok(BinaryOp::NotEq)
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::EqEq, TokenKind::NotEq],
            }),
        }
    })
}
