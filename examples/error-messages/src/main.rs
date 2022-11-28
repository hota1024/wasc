extern crate wasc;

use wasc::lexer::{lex, remove_whitespace_tokens};
use wasc::parser::module::parse_module;
use wasc::parser::token_walker::TokenWalker;

fn main() {
    let input = include_str!("main.wasc");
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);

    println!("{:?}", parse_module(&mut walker));
}
