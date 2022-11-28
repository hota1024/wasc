use crate::{
    ast::item::Item,
    parser::{
        item::item_fn::parse_item_fn,
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
    },
    tokens::TokenKind,
};

pub mod item_fn;

pub fn parse_item(walker: &mut TokenWalker) -> ParseResult<Item> {
    match walker.peek().kind {
        TokenKind::KeywordFn => parse_item_fn(walker, false),
        TokenKind::KeywordExport => {
            walker.next();
            match walker.peek().kind {
                TokenKind::KeywordFn => parse_item_fn(walker, true),
                _ => Err(ParseErr::UnexpectedToken {
                    token: walker.peek().clone(),
                    expected: vec![TokenKind::KeywordFn],
                }),
            }
        }
        _ => Err(ParseErr::UnexpectedToken {
            token: walker.next().clone(),
            expected: vec![TokenKind::KeywordFn],
        }),
    }
}
