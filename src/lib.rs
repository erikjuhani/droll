pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;

pub fn base_prng_engine() -> f64 {
    rand::random::<f64>()
}
