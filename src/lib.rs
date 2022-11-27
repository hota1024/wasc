mod ast;
mod lexer;
mod span;
mod tokens;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    input.to_string()
}

pub fn hello() -> String {
    "Hello, world!".to_string()
}
