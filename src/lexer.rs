use crate::span::Span;
use crate::tokens::{Token, TokenKind};

fn lex_item(input: &[u8], pos: usize) -> Option<(Token, usize)> {
    let char = input[pos];
    let start = pos;
    let mut end = pos;
    let kind;

    match char {
        b';' => {
            end += 1;
            kind = TokenKind::Semi;
        }
        b':' => {
            end += 1;
            kind = TokenKind::Colon;
        }
        b',' => {
            end += 1;
            kind = TokenKind::Comma;
        }

        b'+' => {
            end += 1;
            kind = TokenKind::Plus;
        }
        b'-' => {
            end += 1;
            kind = TokenKind::Minus;
        }
        b'*' => {
            end += 1;
            kind = TokenKind::Star;
        }
        b'/' => {
            end += 1;
            kind = TokenKind::Slash;
        }
        b'=' => {
            end += 1;
            kind = TokenKind::Eq;
        }
        b'(' => {
            end += 1;
            kind = TokenKind::OpenParen;
        }
        b')' => {
            end += 1;
            kind = TokenKind::CloseParen;
        }
        b'{' => {
            end += 1;
            kind = TokenKind::OpenBrace;
        }
        b'}' => {
            end += 1;
            kind = TokenKind::CloseBrace;
        }
        b'0'..=b'9' => {
            while end < input.len() && b"0123456789".contains(&input[end]) {
                end += 1;
            }

            kind = TokenKind::UnsignedInt(
                std::str::from_utf8(&input[start..end])
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        }
        b'a'..=b'z' | b'A'..=b'Z' => {
            end += 1;
            while end < input.len()
                && (b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"
                    .contains(&input[end]))
            {
                end += 1;
            }

            let literal = String::from_utf8(input[start..end].to_vec()).unwrap();

            match literal.as_str() {
                "let" => kind = TokenKind::KeywordLet,
                "i64" => kind = TokenKind::KeywordI64,
                "i32" => kind = TokenKind::KeywordI32,
                "mut" => kind = TokenKind::KeywordMut,
                "fn" => kind = TokenKind::KeywordFn,
                "return" => kind = TokenKind::KeywordReturn,
                _ => kind = TokenKind::Ident(literal),
            }
        }
        b' ' | b'\n' | b'\t' => {
            end += 1;
            kind = TokenKind::Whitespace;
        }
        _ => return None,
    }

    let literal = String::from_utf8(input[start..end].to_vec()).unwrap();

    Some((Token::new(kind, literal, Span::new(start, end)), end))
}

pub fn lex(input: &str) -> Vec<Token> {
    let input = input.as_bytes();
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < input.len() {
        match lex_item(&input, pos) {
            Some((token, p)) => {
                tokens.push(token);
                pos = p;
            }
            None => {
                pos += 1;
            }
        }
    }

    tokens.push(Token::new(
        TokenKind::Eof,
        "".to_string(),
        Span::new(pos, pos),
    ));

    tokens
}

pub fn remove_whitespace_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens.clone();
    tokens.retain(|t| t.kind != TokenKind::Whitespace);

    tokens
}

#[test]
fn test_lex() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        lex(";:+-*/(){}"),
        vec![
            Token::new(TokenKind::Semi, ";".to_string(), Span::new(0, 1)),
            Token::new(TokenKind::Colon, ":".to_string(), Span::new(1, 2)),
            Token::new(TokenKind::Plus, "+".to_string(), Span::new(2, 3)),
            Token::new(TokenKind::Minus, "-".to_string(), Span::new(3, 4)),
            Token::new(TokenKind::Star, "*".to_string(), Span::new(4, 5)),
            Token::new(TokenKind::Slash, "/".to_string(), Span::new(5, 6)),
            Token::new(TokenKind::OpenParen, "(".to_string(), Span::new(6, 7)),
            Token::new(TokenKind::CloseParen, ")".to_string(), Span::new(7, 8)),
            Token::new(TokenKind::OpenBrace, "{".to_string(), Span::new(8, 9)),
            Token::new(TokenKind::CloseBrace, "}".to_string(), Span::new(9, 10)),
            Token::new(TokenKind::Eof, "".to_string(), Span::new(10, 10)),
        ]
    );
    assert_eq!(
        lex("1234567890"),
        vec![
            Token::new(
                TokenKind::UnsignedInt(1234567890),
                "1234567890".to_string(),
                Span::new(0, 10)
            ),
            Token::new(TokenKind::Eof, "".to_string(), Span::new(10, 10)),
        ]
    );
    assert_eq!(
        lex("1 + 2 * 3"),
        vec![
            Token::new(TokenKind::UnsignedInt(1), "1".to_string(), Span::new(0, 1)),
            Token::new(TokenKind::Whitespace, " ".to_string(), Span::new(1, 2)),
            Token::new(TokenKind::Plus, "+".to_string(), Span::new(2, 3)),
            Token::new(TokenKind::Whitespace, " ".to_string(), Span::new(3, 4)),
            Token::new(TokenKind::UnsignedInt(2), "2".to_string(), Span::new(4, 5)),
            Token::new(TokenKind::Whitespace, " ".to_string(), Span::new(5, 6)),
            Token::new(TokenKind::Star, "*".to_string(), Span::new(6, 7)),
            Token::new(TokenKind::Whitespace, " ".to_string(), Span::new(7, 8)),
            Token::new(TokenKind::UnsignedInt(3), "3".to_string(), Span::new(8, 9)),
            Token::new(TokenKind::Eof, "".to_string(), Span::new(9, 9)),
        ]
    );
}

#[test]
fn test_remove_whitespaces() {
    use pretty_assertions::assert_eq;
    let mut tokens = lex("1 + 2 * 3");
    tokens = remove_whitespace_tokens(tokens);

    assert_eq!(
        tokens,
        vec![
            Token::new(TokenKind::UnsignedInt(1), "1".to_string(), Span::new(0, 1)),
            Token::new(TokenKind::Plus, "+".to_string(), Span::new(2, 3)),
            Token::new(TokenKind::UnsignedInt(2), "2".to_string(), Span::new(4, 5)),
            Token::new(TokenKind::Star, "*".to_string(), Span::new(6, 7)),
            Token::new(TokenKind::UnsignedInt(3), "3".to_string(), Span::new(8, 9)),
            Token::new(TokenKind::Eof, "".to_string(), Span::new(9, 9)),
        ]
    );
}
