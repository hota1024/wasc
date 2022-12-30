use crate::{ast::stmt::Stmt, tokens::TokenKind};

use super::{parser_result::ParseResult, token_walker::TokenWalker};

pub mod stmt_let;
pub mod stmt_return;
pub mod stmt_semi;

pub fn parse_stmt(walker: &mut TokenWalker) -> ParseResult<Stmt> {
    match walker.peek().kind {
        TokenKind::KeywordReturn => stmt_return::parse_stmt_return(walker),
        TokenKind::KeywordLet => stmt_let::parse_stmt_let(walker),
        _ => stmt_semi::parse_stmt_semi(walker),
    }
}
