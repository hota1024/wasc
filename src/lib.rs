pub mod ast;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod tokens;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    input.to_string()
}
