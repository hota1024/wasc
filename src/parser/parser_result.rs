use crate::tokens::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErr {
    UnexpectedToken {
        token: Token,
        expected: Vec<TokenKind>,
    },
}

pub type ParseResult<T> = Result<T, ParseErr>;
