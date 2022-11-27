pub mod add;
pub mod atom;
pub mod block;
pub mod mul;
pub mod parse_binary;
pub mod unary;

use crate::ast::expr::Expr;

use self::add::parse_add;

use super::{parser_result::ParseResult, token_walker::TokenWalker};

pub fn parse_expr(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_add(walker)
}
