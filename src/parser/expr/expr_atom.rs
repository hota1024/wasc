use crate::{
    ast::{expr::Expr, lit::Lit},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

use super::{expr_call::parse_expr_call, parse_expr};

pub fn parse_expr_atom(walker: &mut TokenWalker) -> ParseResult<Expr> {
    match walker.peek().kind {
        TokenKind::UnsignedInt(_) => {
            let next = walker.next();
            Ok(Expr::Lit(Lit::from_token(next).unwrap()))
        }
        TokenKind::UnsignedFloat(_) => {
            let next = walker.next();
            Ok(Expr::Lit(Lit::from_token(next).unwrap()))
        }
        TokenKind::OpenParen => {
            walker.next();
            let expr = parse_expr(walker)?;
            walker.expect_next_token(TokenKind::CloseParen)?;

            Ok(expr)
        }
        TokenKind::Ident(_) => {
            if walker.peek_over(2).kind == TokenKind::OpenParen {
                parse_expr_call(walker)
            } else {
                let next = walker.next();
                Ok(Expr::Lit(Lit::from_token(&next).unwrap()))
            }
        }
        TokenKind::KeywordTrue | TokenKind::KeywordFalse => {
            let next = walker.next();
            Ok(Expr::Lit(Lit::from_token(next).unwrap()))
        }
        _ => Err(ParseErr::UnexpectedToken {
            token: walker.peek().clone(),
            expected: vec![
                TokenKind::UnsignedInt(0),
                TokenKind::UnsignedFloat(0.0),
                TokenKind::Ident("".to_string()),
            ],
        }),
    }
}
