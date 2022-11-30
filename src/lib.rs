pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;
pub mod sexpr;
pub mod span;
pub mod tokens;

use compiler::Compiler;
use lexer::{lex, remove_whitespace_tokens};
use parser::{module::parse_module, token_walker::TokenWalker};
use sexpr::encoder::Encoder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    let tokens = remove_whitespace_tokens(lex(input));
    let mut walker = TokenWalker::new(tokens);
    let module = parse_module(&mut walker).unwrap();

    let compiler = Compiler::new();
    let sexpr = compiler.compile_module(module);

    let encoder = Encoder::default();

    encoder.encode(&sexpr)
}
