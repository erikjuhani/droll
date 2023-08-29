use droll::{base_prng_engine, interpreter::eval, parser::parse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn roll(input: &str) -> Result<isize, String> {
    match parse(input) {
        Ok(parse_tree) => Ok(eval(base_prng_engine)(parse_tree)),
        Err(e) => Err(e),
    }
}
