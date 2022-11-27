use crate::tokens::{Token, TokenKind};

pub mod LitIdent;
pub mod LitUnsignedInt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    LitUnsignedInt(LitUnsignedInt::LitUnsignedInt),
    LitIdent(LitIdent::LitIdent),
}

impl Lit {
    pub fn from_token(token: &Token) -> Option<Self> {
        match &token.kind {
            TokenKind::UnsignedInt(value) => {
                Some(Self::LitUnsignedInt(LitUnsignedInt::LitUnsignedInt {
                    value: *value,
                }))
            }
            TokenKind::Ident(ident) => Some(Self::LitIdent(LitIdent::LitIdent {
                ident: ident.clone(),
            })),
            _ => None,
        }
    }
}
