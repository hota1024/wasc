pub mod expr_add;
pub mod expr_as;
pub mod expr_assign;
pub mod expr_atom;
pub mod expr_block;
pub mod expr_call;
pub mod expr_mul;
pub mod expr_not;
pub mod expr_prefix_minus;
pub mod parse_binary;

use crate::ast::expr::Expr;

use super::{parser_result::ParseResult, token_walker::TokenWalker};

pub fn parse_expr(walker: &mut TokenWalker) -> ParseResult<Expr> {
    self::expr_assign::parse_expr_assign(walker)
}
