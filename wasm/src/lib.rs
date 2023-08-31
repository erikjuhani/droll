use droll::{base_prng_engine, interpreter::eval, parser::parse};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Calculates the total sum from the provided dice notation, the roll result.
/// @example
/// roll("1d20+10"); // e.g. 27
pub fn roll(input: &str) -> Result<isize, String> {
    match parse(input) {
        Ok(parse_tree) => Ok(eval(base_prng_engine)(parse_tree)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        let tests = [("1d20+10", (11, 30))];

        tests.iter().for_each(|(notation, (min, max))| {
            let result = roll(notation).unwrap();

            assert!(
                result >= *min && result <= *max,
                "roll result {} was not in the range of min: {}, max: {}",
                result,
                min,
                max
            );
        })
    }
}
