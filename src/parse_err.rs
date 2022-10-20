use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErr {
    UnexpectedToken { token: Token, expected: String },
}

impl ParseErr {
    pub fn unexpected_token(token: Token, expected: String) -> Self {
        Self::UnexpectedToken { token, expected }
    }
}
