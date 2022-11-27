extern crate wasc;

use wasc::lexer::{lex, remove_whitespace_tokens};
use wasc::parser::item::parse_item;
use wasc::parser::token_walker::TokenWalker;

fn main() {
    let input = "export fn main(a: i32, b: i32): i32 {return 1 + 2;}";
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);

    println!("{:?}", parse_item(&mut walker));
}
