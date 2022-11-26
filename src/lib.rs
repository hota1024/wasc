mod ast;
mod lexer;
mod parsers;
mod span;
mod tokens;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(input: &str) -> String {
    input.to_string()
}
