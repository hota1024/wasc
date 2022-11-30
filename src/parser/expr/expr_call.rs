use crate::{
    ast::{
        expr::{expr_call::ExprCall, Expr},
        lit::lit_ident::LitIdent,
    },
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::parse_expr;

pub fn parse_expr_call(walker: &mut TokenWalker) -> ParseResult<Expr> {
    let fn_name = LitIdent::from_token(walker.next()).unwrap();
    let args = parse_args(walker)?;

    Ok(Expr::ExprCall(ExprCall { fn_name, args }))
}

pub fn parse_args(walker: &mut TokenWalker) -> ParseResult<Vec<Expr>> {
    walker.expect_next_token(TokenKind::OpenParen)?;
    let mut args = vec![];

    while walker.peek().kind != TokenKind::CloseParen {
        args.push(parse_expr(walker)?);

        if walker.peek().kind == TokenKind::Comma {
            walker.next();
        } else {
            break;
        }
    }

    walker.expect_next_token(TokenKind::CloseParen)?;

    Ok(args)
}
