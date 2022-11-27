use crate::{ast::ty::Ty, tokens::TokenKind};

use super::{
    parser_result::{ParseErr, ParseResult},
    token_walker::TokenWalker,
};

pub fn parse_ty(walker: &mut TokenWalker) -> ParseResult<Ty> {
    let token = walker.next();

    match token.kind {
        TokenKind::KeywordI64 => Ok(Ty::TyInt64),
        TokenKind::KeywordI32 => Ok(Ty::TyInt32),
        TokenKind::KeywordF64 => Ok(Ty::TyFloat64),
        TokenKind::KeywordF32 => Ok(Ty::TyFloat32),
        _ => Err(ParseErr::UnexpectedToken {
            token: token.clone(),
            expected: vec![
                TokenKind::KeywordI64,
                TokenKind::KeywordI32,
                TokenKind::KeywordF64,
                TokenKind::KeywordF32,
            ],
        }),
    }
}
