use lexer::{lex, remove_whitespace_tokens};
use parsers::parse;
use sexpr::Expr;
use token_walker::TokenWalker;
use wasm_bindgen::prelude::*;

mod ast;
mod ast_to_sexpr;
mod lexer;
mod parse_err;
mod parse_utils;
mod parser_common;
mod parsers;
mod sexpr;
mod span;
mod token_walker;
mod tokens;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);
    let ast = parse(&mut walker).unwrap();
    let expr = ast_to_sexpr::ast_to_sexpr(&ast).unwrap();

    return Expr::encode(&expr);
}
