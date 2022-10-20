use crate::{
    lexer::{lex, remove_whitespace_tokens},
    parse_err::ParseErr,
    span::Span,
    tokens::{Token, TokenKind},
};

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

    pub fn next(&mut self) -> &Token {
        self.pos += 1;
        let token = self.tokens.get(self.pos as usize).unwrap();

        token
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get((self.pos + 1) as usize).unwrap()
    }

    pub fn expect_next_token(&mut self, kind: TokenKind) -> Result<&Token, ParseErr> {
        let token = self.next();

        if token.kind != kind {
            Err(ParseErr::unexpected_token(
                token.clone(),
                format!("{:?}", kind),
            ))
        } else {
            Ok(token)
        }
    }

    pub fn expect_current_token(&mut self, kind: TokenKind) -> Result<&Token, ParseErr> {
        let token = self.current();

        if token.kind != kind {
            Err(ParseErr::unexpected_token(
                token.clone(),
                format!("{:?}", kind),
            ))
        } else {
            Ok(token)
        }
    }
}

#[test]
fn test_token_walker_next() {
    use pretty_assertions::assert_eq;

    let tokens = remove_whitespace_tokens(lex("1 + 2"));
    let mut walker = TokenWalker::new(tokens);

    assert_eq!(walker.next().kind, TokenKind::UnsignedInt(1));
    assert_eq!(walker.next().kind, TokenKind::Plus);
    assert_eq!(walker.next().kind, TokenKind::UnsignedInt(2));
}

#[test]
fn test_token_walker_peek() {
    use pretty_assertions::assert_eq;

    let tokens = remove_whitespace_tokens(lex("1 + 2"));
    let mut walker = TokenWalker::new(tokens);

    assert_eq!(walker.peek().kind, TokenKind::UnsignedInt(1));
    assert_eq!(walker.next().kind, TokenKind::UnsignedInt(1));
    assert_eq!(walker.peek().kind, TokenKind::Plus);
    assert_eq!(walker.next().kind, TokenKind::Plus);
    assert_eq!(walker.peek().kind, TokenKind::UnsignedInt(2));
    assert_eq!(walker.next().kind, TokenKind::UnsignedInt(2));
}

#[test]
fn test_token_walker_expect() {
    use pretty_assertions::assert_eq;

    let tokens = remove_whitespace_tokens(lex("1 + 2"));
    let mut walker = TokenWalker::new(tokens);

    assert_eq!(
        walker
            .expect_next_token(TokenKind::UnsignedInt(1))
            .unwrap()
            .kind,
        TokenKind::UnsignedInt(1)
    );
    assert_eq!(
        walker.expect_next_token(TokenKind::Plus).unwrap().kind,
        TokenKind::Plus
    );
    assert_eq!(
        walker
            .expect_next_token(TokenKind::UnsignedInt(2))
            .unwrap()
            .kind,
        TokenKind::UnsignedInt(2)
    );
}
