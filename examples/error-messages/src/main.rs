extern crate wasc;

use wasc::lexer::{lex, remove_whitespace_tokens};
use wasc::parser::expr::parse_expr;
use wasc::parser::token_walker::TokenWalker;

fn main() {
    let input = "1 + 2 * -3";
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);

    println!("{:?}", parse_expr(&mut walker));
}
