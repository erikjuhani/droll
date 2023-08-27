use std::{
    iter::{self, Peekable},
    str::Chars,
};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents the different types of tokens that can be parsed from the input.
pub enum Token {
    /// Represents a numeric value.
    Number(usize),
    /// Represents the addition operator.
    Plus,
    /// Represents the subtraction operator.
    Minus,
    /// Represents a die token in dice notation.
    Die,
}

/// Performs lexical analysis of the provided string, transforming it into a vector of [`Token`]s.
///
/// Lexical analysis, often referred to as tokenization, is the process of breaking down a string
/// into meaningful units called tokens. In this context:
///
/// - A [`Token`] represents a type of dice notation unit parsed from the input string.
///
/// - The function returns a [`Result`] to handle potential errors:
///   - `Err(String)`: Signifies that unexpected or invalid characters were encountered during
///     tokenization.
///   - `Ok(Vec<Token>)`: Indicates a successful tokenization process, producing a vector
///     containing the identified dice notation tokens.
///
/// # Example
///
/// Basic Usage:
///
/// ```
/// use droll::lexer::{Token,lex};
///
/// let dice_notation = "1d20+10";
/// let tokens = lex(dice_notation).unwrap();
///
/// assert_eq!(vec![Token::Number(1), Token::Die, Token::Number(20), Token::Plus, Token::Number(10)], tokens);
/// ```
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&char) = chars.to_owned().peek() {
        tokens.push(parse_token(char, &mut chars)?);
    }

    Ok(tokens)
}

/// Converts a [`char`] into an operator token.
fn operator_token(char: char, chars: &mut Peekable<Chars>) -> Result<Token, String> {
    chars.next();
    match char {
        '+' => Ok(Token::Plus),
        '-' => Ok(Token::Minus),
        'd' => Ok(Token::Die),
        c => error_unexpected_character(c),
    }
}

/// Converts consecutive characters into a number token, extending as long as the following
/// characters are ASCII digits.
fn number_token(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    match iter::from_fn(|| chars.next_if(|c| c.is_ascii_digit()))
        .collect::<String>()
        .parse::<usize>()
    {
        Ok(number) => Ok(Token::Number(number)),
        parse_err => Err(parse_err.unwrap_err().to_string()),
    }
}

/// Converts a [`char`] into a [`Token`].
fn parse_token(char: char, chars: &mut Peekable<Chars>) -> Result<Token, String> {
    match char {
        '1'..='9' => number_token(chars),
        '+' | '-' | 'd' => operator_token(char, chars),
        c => error_unexpected_character(c),
    }
}

fn error_unexpected_character<T>(c: char) -> Result<T, String> {
    Err(format!("Unexpected character: {}", c))
}

#[test]
fn test_lex_valid() {
    let tests = [
        (
            "+-1234567890d",
            vec![
                Token::Plus,
                Token::Minus,
                Token::Number(1234567890),
                Token::Die,
            ],
        ),
        ("d20", vec![Token::Die, Token::Number(20)]),
        (
            "2d20",
            vec![Token::Number(2), Token::Die, Token::Number(20)],
        ),
        (
            "2d20+1d8",
            vec![
                Token::Number(2),
                Token::Die,
                Token::Number(20),
                Token::Plus,
                Token::Number(1),
                Token::Die,
                Token::Number(8),
            ],
        ),
        (
            "d20-10",
            vec![
                Token::Die,
                Token::Number(20),
                Token::Minus,
                Token::Number(10),
            ],
        ),
    ];

    tests.iter().for_each(|(input, expected)| {
        let actual = lex(input).unwrap();
        assert_eq!(actual, *expected, "for input `{:#?}`", input);
    })
}
