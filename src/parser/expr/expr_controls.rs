use crate::{
    ast::{
        expr::{expr_call::ExprCall, Expr},
        lit::lit_ident::LitIdent,
    },
    parser::{parser_result::ParseResult, token_walker::TokenWalker},
    tokens::TokenKind,
};

use super::{parse_expr, expr_atom::parse_expr_atom};

pub fn parse_expr_controls(walker: &mut TokenWalker) -> ParseResult<Expr> {
    match walker.peek().kind {
        TokenKind::If
        _ => {
            parse_expr_atom()
        }
    }
}
