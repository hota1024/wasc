pub mod expr_add;
pub mod expr_as;
pub mod expr_atom;
pub mod expr_block;
pub mod expr_call;
pub mod expr_mul;
pub mod expr_unary;
pub mod parse_binary;

use crate::ast::expr::Expr;

use self::expr_add::parse_expr_add;

use super::{parser_result::ParseResult, token_walker::TokenWalker};

pub fn parse_expr(walker: &mut TokenWalker) -> ParseResult<Expr> {
    parse_expr_add(walker)
}
