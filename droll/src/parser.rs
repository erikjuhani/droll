use std::{iter::Peekable, slice::Iter};

use crate::ast::{binary_expr, numeric_literal, unary_expr, Expr, Operator};
use crate::lexer;
use crate::lexer::Token;

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
    Ok(parse_expr(&mut lexer::lex(input)?.iter().peekable(), 0))
}

fn token_to_operator(token: Token) -> Operator {
    match token {
        Token::Plus => Operator::Plus,
        Token::Minus => Operator::Minus,
        Token::Die => Operator::Die,
        op => panic!("bad token {:?}", op),
    }
}

fn infix_binding_power(token: Token) -> (u8, u8) {
    match token {
        Token::Plus | Token::Minus => (1, 2),
        Token::Die => (3, 4),
        token => panic!("bad token {:?}", token),
    }
}

fn prefix_binding_power(token: Token) -> ((), u8) {
    match token {
        Token::Plus | Token::Minus => ((), 5),
        Token::Die => ((), 7),
        token => panic!("bad token {:?}", token),
    }
}

fn parse_unary_expr(tokens: &mut Peekable<Iter<'_, Token>>, op_token: Token) -> Expr {
    let ((), r_bp) = prefix_binding_power(op_token);
    let rhs = parse_expr(tokens, r_bp);
    unary_expr(rhs, token_to_operator(op_token))
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>, min_binding_power: u8) -> Expr {
    let mut lhs = match tokens.next() {
        Some(&Token::Number(n)) => numeric_literal(n),
        Some(&op_token) => parse_unary_expr(tokens, op_token),
        token => panic!("bad token {:?}", token),
    };

    loop {
        match tokens.peek() {
            Some(&&token) => {
                let (l_bp, r_bp) = infix_binding_power(token);
                if l_bp < min_binding_power {
                    break;
                }

                tokens.next();

                let rhs = parse_expr(tokens, r_bp);

                lhs = binary_expr(lhs, rhs, token_to_operator(token))
            }
            None => break,
        }
    }

    lhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tests = [
            ("1d20", "(d 1 20)"),
            ("-1d20", "(d (- 1) 20)"),
            ("d20", "(d 20)"),
            ("-d20", "(- (d 20))"),
            ("3d6+10", "(+ (d 3 6) 10)"),
            ("3-d6", "(- 3 (d 6))"),
            ("d3-2", "(- (d 3) 2)"),
            ("-2-d8", "(- (- 2) (d 8))"),
            ("+1--d3", "(- (+ 1) (- (d 3)))"),
            ("1d20+2d3", "(+ (d 1 20) (d 2 3))"),
        ];

        tests.iter().for_each(|(input, expected)| {
            assert_eq!(
                parse(input).unwrap().to_string(),
                *expected,
                "for input `{:#?}`",
                input
            );
        })
    }
}
