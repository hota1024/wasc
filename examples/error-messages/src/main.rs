extern crate wasc;

use wasc::lexer::{lex, remove_whitespace_tokens};
use wasc::parser::module::parse_module;
use wasc::parser::token_walker::TokenWalker;

fn main() {
    let input = "export fn main(): i32 {}\nexport fn test(): i32 {}";
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);

    println!("{:?}", parse_module(&mut walker));
}
