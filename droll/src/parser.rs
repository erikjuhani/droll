use std::{iter::Peekable, slice::Iter};

use crate::ast::{binary_expr, unary_expr, Expr, Operator};
use crate::lexer;

/// Defines the maximum precedence.
const MAX_PRECEDENCE: usize = 2;

/// First parse function performs a lexical analysis of the given input string to transform the
/// input into readable tokens then a parse tree is generated from the tokens using
/// operator-precedence parsing.
///
/// The parser uses the following grammar (EBNF):
///
/// <expr> ::= <roll-expr>
///          | <expr> '+' <expr>
///          | <expr> '-' <expr>
///
/// <roll-expr> ::= <primary>
///               | <expr 'd' <expr>
///
/// <primary> ::= <number>
///             | '+' <primary>
///             | '-' <primary>
///             | 'd' <expr>
///
/// <number> ::= <non-zero-digit> { <digit> }
/// <digit> ::= '0' .. '9'
/// <non-zero-digit> ::= '1' .. '9'
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// use droll::parser::{parse};
/// use droll::ast::{binary_roll_expr, binary_expr, numeric_literal, Operator};
///
/// let dice_notation = "1d20+10";
/// let parse_tree = parse(dice_notation).unwrap();
///
/// assert_eq!(binary_expr(binary_roll_expr(1, 20), numeric_literal(10), Operator::Plus), parse_tree);
/// ```
pub fn parse(input: &str) -> Result<Expr, String> {
    parse_expr(&mut lexer::lex(input)?.iter().peekable(), 0)
}

fn token_precedence(token: lexer::Token) -> usize {
    match token {
        lexer::Token::Plus | lexer::Token::Minus => 0,
        lexer::Token::Die => 1,
        _ => MAX_PRECEDENCE,
    }
}

fn token_to_operator(token: lexer::Token) -> Result<Operator, String> {
    match token {
        lexer::Token::Plus => Ok(Operator::Plus),
        lexer::Token::Minus => Ok(Operator::Minus),
        lexer::Token::Die => Ok(Operator::Die),
        op => Err(format!("Unknown operator token {:?}", op)),
    }
}

fn parse_expr(
    tokens: &mut Peekable<Iter<'_, lexer::Token>>,
    current_precedence: usize,
) -> Result<Expr, String> {
    if current_precedence > MAX_PRECEDENCE {
        return parse_primary(tokens);
    }

    let mut left = parse_expr(tokens, current_precedence + 1)?;

    if let Some(&&next_token) = tokens.peek() {
        if token_precedence(next_token) == current_precedence {
            tokens.next();
            left = binary_expr(
                left,
                parse_expr(tokens, current_precedence)?,
                token_to_operator(next_token)?,
            );
        }
    }

    return Ok(left);
}

fn parse_primary(tokens: &mut Peekable<Iter<'_, lexer::Token>>) -> Result<Expr, String> {
    match tokens.peek() {
        Some(lexer::Token::Number(n)) => {
            tokens.next();
            Ok(Expr::NumericLiteral(*n))
        }
        Some(lexer::Token::Die) => {
            tokens.next();
            match tokens.peek() {
                Some(lexer::Token::Die) => {
                    return Err("Syntax error, found 'd' token directly after 'd' token".to_string())
                }
                None => {
                    return Err(
                        "Unexpected end of input, expecting token after 'd' token".to_string()
                    )
                }
                _ => (),
            }
            Ok(unary_expr(parse_expr(tokens, 0)?, Operator::Die))
        }
        Some(lexer::Token::Minus) => {
            tokens.next();
            if let None = tokens.peek() {
                return Err("Unexpected end of input, expecting token after '-' token".to_string());
            }
            Ok(unary_expr(parse_primary(tokens)?, Operator::Minus))
        }
        Some(lexer::Token::Plus) => {
            tokens.next();
            if let None = tokens.peek() {
                return Err("Unexpected end of input, expecting token after '+' token".to_string());
            }
            Ok(unary_expr(parse_primary(tokens)?, Operator::Plus))
        }
        _ => Err("Unexpected end of input".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{
        binary_expr, binary_roll_expr, numeric_literal, unary_expr, unary_roll_expr, Operator,
    };

    #[test]
    fn test_parse() {
        let tests = [
            ("1d20", binary_roll_expr(1, 20)),
            (
                "-1d20",
                binary_expr(
                    unary_expr(numeric_literal(1), Operator::Minus),
                    numeric_literal(20),
                    Operator::Die,
                ),
            ),
            ("d20", unary_roll_expr(20)),
            ("-d20", unary_expr(unary_roll_expr(20), Operator::Minus)),
            (
                "3d6+10",
                binary_expr(binary_roll_expr(3, 6), numeric_literal(10), Operator::Plus),
            ),
            (
                "3-d6",
                binary_expr(numeric_literal(3), unary_roll_expr(6), Operator::Minus),
            ),
            (
                "-2-d8",
                binary_expr(
                    unary_expr(numeric_literal(2), Operator::Minus),
                    unary_roll_expr(8),
                    Operator::Minus,
                ),
            ),
            (
                "+1--d3",
                binary_expr(
                    unary_expr(numeric_literal(1), Operator::Plus),
                    unary_expr(unary_roll_expr(3), Operator::Minus),
                    Operator::Minus,
                ),
            ),
            (
                "1d20+2d3",
                binary_expr(
                    binary_roll_expr(1, 20),
                    binary_roll_expr(2, 3),
                    Operator::Plus,
                ),
            ),
        ];

        tests.iter().for_each(|(input, expected)| {
            assert_eq!(parse(input).unwrap(), *expected, "for input `{:#?}`", input);
        })
    }
}
