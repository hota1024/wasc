use crate::{
    parser::parser_result::ParseErr,
    tokens::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitIdent {
    pub ident: String,
}

impl LitIdent {
    pub fn from_token(token: &Token) -> Result<Self, ParseErr> {
        match &token.kind {
            TokenKind::Ident(ident) => Ok(Self {
                ident: ident.clone(),
            }),
            _ => Err(ParseErr::UnexpectedToken {
                token: token.clone(),
                expected: vec![TokenKind::Ident("".to_string())],
            }),
        }
    }
}
