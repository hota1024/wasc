use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Semi,  // ;
    Colon, // :
    Comma, // ,

    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }

    Int(i64),
    Ident(String),

    Whitespace,

    Unknown,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String, span: Span) -> Self {
        Self {
            kind,
            literal,
            span,
        }
    }
}
