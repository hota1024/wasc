use crate::tokens::{Token, TokenKind};

pub mod lit_ident;
pub mod lit_unsigned_int;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    LitUnsignedInt(lit_unsigned_int::LitUnsignedInt),
    LitIdent(lit_ident::LitIdent),
}

impl Lit {
    pub fn from_token(token: &Token) -> Option<Self> {
        match &token.kind {
            TokenKind::UnsignedInt(value) => {
                Some(Self::LitUnsignedInt(lit_unsigned_int::LitUnsignedInt {
                    value: *value,
                }))
            }
            TokenKind::Ident(ident) => Some(Self::LitIdent(lit_ident::LitIdent {
                ident: ident.clone(),
            })),
            _ => None,
        }
    }
}
