use crate::{ast::module::Module, tokens::TokenKind};

use super::{item::parse_item, parser_result::ParseResult, token_walker::TokenWalker};

pub fn parse_module(walker: &mut TokenWalker) -> ParseResult<Module> {
    let mut items = vec![];

    while walker.peek().kind != TokenKind::Eof {
        items.push(parse_item(walker)?);
    }

    Ok(Module { items })
}
