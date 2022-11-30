use crate::tokens::{Token, TokenKind};

use super::parser_result::ParseErr;

pub struct TokenWalker {
    tokens: Vec<Token>,
    pos: i64,
}

impl TokenWalker {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: -1 }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.pos as usize]
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get((self.pos + 1) as usize).unwrap()
    }

    pub fn peek_over(&self, n: i64) -> &Token {
        self.tokens.get((self.pos + n) as usize).unwrap()
    }

    pub fn next(&mut self) -> &Token {
        self.pos += 1;
        let token = self.tokens.get(self.pos as usize).unwrap();

        token
    }

    pub fn get_pos(&self) -> i64 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: i64) {
        self.pos = pos;
    }

    pub fn expect_next_token(&mut self, kind: TokenKind) -> Result<&Token, ParseErr> {
        let token = self.next();

        if token.kind == kind {
            Ok(token)
        } else {
            Err(ParseErr::UnexpectedToken {
                token: token.clone(),
                expected: vec![kind],
            })
        }
    }

    pub fn expect_current_token(&mut self, kind: TokenKind) -> Result<&Token, ParseErr> {
        let token = self.current();

        if token.kind == kind {
            Ok(token)
        } else {
            Err(ParseErr::UnexpectedToken {
                token: token.clone(),
                expected: vec![kind],
            })
        }
    }
}
