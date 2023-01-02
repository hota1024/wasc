use crate::{
    ast::expr::{
        expr_binary::{AssignOp, BinaryOp},
        Expr,
    },
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_equality::parse_expr_equality, parse_binary::parse_binary_expr};

pub fn parse_expr_assign(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_binary_expr(walker, parse_expr_equality, |walker| {
        let peek = walker.peek();

        match peek.kind {
            TokenKind::Eq => {
                walker.next();
                Ok(BinaryOp::Assign)
            }
            TokenKind::PlusEq => {
                walker.next();
                Ok(BinaryOp::AssignOp(AssignOp::Add))
            }
            TokenKind::MinusEq => {
                walker.next();
                Ok(BinaryOp::AssignOp(AssignOp::Sub))
            }
            TokenKind::StarEq => {
                walker.next();
                Ok(BinaryOp::AssignOp(AssignOp::Mul))
            }
            TokenKind::SlashEq => {
                walker.next();
                Ok(BinaryOp::AssignOp(AssignOp::Div))
            }
            _ => Err(ParseErr::UnexpectedToken {
                token: peek.clone(),
                expected: vec![TokenKind::Eq],
            }),
        }
    })
}
