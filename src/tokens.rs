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

    Eq, // =

    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }

    UnsignedInt(u64),
    Ident(String),

    KeywordLet,    // let
    KeywordI32,    // i32
    KeywordI64,    // i64
    KeywordMut,    // mut
    KeywordFn,     // fn
    KeywordReturn, // return
    KeywordExport, // export

    Whitespace,

    Unknown,
    Eof,
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
