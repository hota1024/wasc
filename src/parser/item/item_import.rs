use crate::{
    ast::{
        item::{
            item_import::{
                ImportItem, ImportItemFn, ImportItemFnParam, ImportItemKind, ItemImport,
            },
            Item,
        },
        lit::lit_ident::LitIdent,
    },
    parser::{
        parser_result::{ParseErr, ParseResult},
        token_walker::TokenWalker,
        ty::parse_ty,
    },
    tokens::TokenKind,
};

pub fn parse_item_import(walker: &mut TokenWalker) -> ParseResult<Item> {
    walker.expect_next_token(TokenKind::KeywordImport);
    let mod_name = LitIdent::from_token(walker.next())?;
    let mut items = vec![];

    walker.expect_next_token(TokenKind::OpenBrace);

    while walker.peek().kind != TokenKind::CloseBrace {
        let item = parse_import_item(walker)?;
        items.push(item);
    }

    walker.expect_next_token(TokenKind::CloseBrace);

    Ok(Item::ItemImport(ItemImport { mod_name, items }))
}

fn parse_import_item(walker: &mut TokenWalker) -> ParseResult<ImportItem> {
    match walker.peek().kind {
        TokenKind::KeywordFn => parse_import_item_fn(walker),
        _ => Err(ParseErr::UnexpectedToken {
            token: walker.next().clone(),
            expected: vec![TokenKind::KeywordFn],
        }),
    }
}

fn parse_import_item_fn(walker: &mut TokenWalker) -> ParseResult<ImportItem> {
    // fn <name>(<type0>, <type1>, <type...>...);
    walker.expect_next_token(TokenKind::KeywordFn);
    let name = LitIdent::from_token(walker.next())?;
    let mut params = vec![];

    walker.expect_next_token(TokenKind::OpenParen);

    while walker.peek().kind != TokenKind::CloseParen {
        params.push(parse_import_item_fn_param(walker)?);

        if walker.peek().kind == TokenKind::Comma {
            walker.next();
        } else {
            break;
        }
    }

    walker.expect_next_token(TokenKind::CloseParen);

    let ret_ty = if walker.peek().kind == TokenKind::Colon {
        walker.next();
        Some(parse_ty(walker)?)
    } else {
        None
    };

    walker.expect_next_token(TokenKind::Semi);

    Ok(ImportItem {
        kind: ImportItemKind::Fn(ImportItemFn {
            name,
            params,
            ret_ty,
        }),
    })
}

fn parse_import_item_fn_param(walker: &mut TokenWalker) -> ParseResult<ImportItemFnParam> {
    let ty = parse_ty(walker)?;

    Ok(ImportItemFnParam { ty })
}
