use crate::{
    ast::{expr::Expr, lit::Lit},
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

pub fn parse_atom(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let next = walker.next();

    match next.kind {
        TokenKind::UnsignedInt(_) => Ok(Expr::Lit(Lit::from_token(&next).unwrap())),
        _ => Err(ParseErr::UnexpectedToken {
            token: next.clone(),
            expected: vec![TokenKind::UnsignedInt(0)],
        }),
    }
}
