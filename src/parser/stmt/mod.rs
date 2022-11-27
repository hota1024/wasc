use crate::{ast::stmt::Stmt, tokens::TokenKind};

use super::{parser_result::ParseResult, token_walker::TokenWalker};

pub mod semi;
pub mod stmt_return;

pub fn parse_stmt(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    match walker.peek().kind {
        TokenKind::KeywordReturn => stmt_return::parse_return(walker),
        _ => semi::parse_semi(walker),
    }
}
