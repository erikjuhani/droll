use std::ops::Neg;

use crate::ast::{Expr, Operator};

/// Evaluates the passed parse tree ([`Expr`]) recursively. [`eval`] is a high-order function that
/// takes, as it's first argument, a random number generator engine. Calling the function will
/// output the actual evaluator that takes the parse tree ([`Expr`]) as an argument.
///
/// The random number generator engine is used in the roll calculations.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// use droll::parser::{parse};
/// use droll::interpreter::{eval};
///
/// let dice_notation = "1d20+10";
/// let parse_tree = parse(dice_notation).unwrap();
/// let prng_engine = || 1f64; // Engine to always roll highest result.
/// let evaluation = eval(prng_engine)(parse_tree);
///
/// assert_eq!(30, evaluation);
/// ```
pub fn eval(prng: fn() -> f64) -> impl Fn(Expr) -> isize {
    move |ast: Expr| -> isize {
        let e = eval(prng);
        match ast {
            Expr::NumericLiteral(n) => n as isize,
            Expr::Binary(lhs, rhs, op) => match op {
                Operator::Die => calc_roll(prng)(e(*lhs), e(*rhs)),
                Operator::Plus => e(*lhs) + e(*rhs),
                Operator::Minus => e(*lhs) - e(*rhs),
            },
            Expr::Unary(rhs, op) => match op {
                Operator::Die => calc_roll(prng)(1, e(*rhs)),
                Operator::Plus => e(*rhs),
                Operator::Minus => e(*rhs).neg(),
            },
        }
    }
}

fn calc_roll(prng: fn() -> f64) -> impl Fn(isize, isize) -> isize {
    move |amount: isize, sides: isize| -> isize {
        (amount as f64 * (prng() * sides as f64).round().max(1.0)) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        binary_expr, binary_roll_expr, numeric_literal, unary_expr, unary_roll_expr, Operator,
    };
    #[test]
    fn test_eval() {
        let tests = [
            (binary_roll_expr(1, 20), 20),
            (
                binary_expr(binary_roll_expr(3, 6), numeric_literal(10), Operator::Plus),
                28,
            ),
            (
                binary_expr(
                    binary_roll_expr(1, 20),
                    binary_roll_expr(2, 3),
                    Operator::Plus,
                ),
                26,
            ),
            (unary_roll_expr(6), 6),
            (unary_expr(numeric_literal(1), Operator::Minus), -1),
            (
                unary_expr(
                    unary_expr(numeric_literal(1), Operator::Minus),
                    Operator::Plus,
                ),
                -1,
            ),
        ];

        tests.iter().for_each(|(input, expected)| {
            assert_eq!(
                eval(|| 1.0)(input.clone()),
                *expected,
                "for input `{:#?}`",
                input
            );
        })
    }
}
