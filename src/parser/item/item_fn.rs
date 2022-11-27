use crate::{
    ast::{
        item::{
            Item,
            ItemFn::{FnParam, ItemFn},
        },
        lit::LitIdent::LitIdent,
    },
    parser::{
        expr::block::parse_block, parser_result::ParseResult, token_walker::TokenWalker,
        ty::parse_ty,
    },
    tokens::TokenKind,
};

pub fn parse_fn(walker: &mut TokenWalker, exported: bool) -> ParseResult<Item> {
    walker.expect_next_token(TokenKind::KeywordFn)?;
    let name = LitIdent::from_token(walker.next())?;
    let params = parse_fn_params(walker)?;

    walker.expect_next_token(TokenKind::Colon)?;
    let ret_ty = parse_ty(walker)?;

    let body = parse_block(walker)?;

    Ok(Item::ItemFn(ItemFn {
        exported,
        name,
        params,
        ret_ty,
        body,
    }))
}

pub fn parse_fn_params(walker: &mut TokenWalker) -> ParseResult<Vec<FnParam>> {
    let mut params = vec![];
    walker.expect_next_token(TokenKind::OpenParen)?;

    while walker.peek().kind != TokenKind::CloseParen {
        params.push(parse_fn_param(walker)?);

        if walker.peek().kind == TokenKind::Comma {
            walker.next();
        } else {
            break;
        }
    }

    walker.expect_next_token(TokenKind::CloseParen)?;

    return Ok(params);
}

pub fn parse_fn_param(walker: &mut TokenWalker) -> ParseResult<FnParam> {
    // <name>
    let name = LitIdent::from_token(walker.next())?;

    // :
    walker.expect_next_token(TokenKind::Colon)?;

    // <type>
    let ty = parse_ty(walker)?;

    Ok(FnParam { name, ty })
}
