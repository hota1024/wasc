use crate::{
    ast::{expr::Expr, lit::Lit},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::expr_call::parse_expr_call;

pub fn parse_expr_atom(walker: &mut TokenWalker) -> ParseResult<Expr> {
    match walker.peek().kind {
        TokenKind::UnsignedInt(_) => {
            let next = walker.next();
            Ok(Expr::Lit(Lit::from_token(next).unwrap()))
        }
        TokenKind::Ident(_) => {
            if walker.peek_over(2).kind == TokenKind::OpenParen {
                parse_expr_call(walker)
            } else {
                let next = walker.next();
                Ok(Expr::Lit(Lit::from_token(&next).unwrap()))
            }
        }
        _ => Err(ParseErr::UnexpectedToken {
            token: walker.peek().clone(),
            expected: vec![TokenKind::UnsignedInt(0)],
        }),
    }
}
