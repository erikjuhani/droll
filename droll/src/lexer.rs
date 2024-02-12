use std::{
    iter::{self, Peekable},
    str::Chars,
};

/// Represents the different types of tokens that can be parsed from the input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Integer(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Die,
    FudgeDie,
    PercentileDie,
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
/// assert_eq!(vec![Token::Integer(1), Token::Die, Token::Integer(20), Token::Plus, Token::Integer(10)], tokens);
/// ```
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(&char) = chars.to_owned().peek() {
        tokens.push(parse(char, &mut chars)?);
    }

    Ok(tokens)
}

/// Converts consecutive characters into an integer token, extending as long as the following
/// characters are ASCII digits.
fn parse_integer_token(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    iter::from_fn(|| chars.next_if(|c| c.is_ascii_digit()))
        .collect::<String>()
        .parse::<u64>()
        .map(Token::Integer)
        .map_err(|err| format!("Failed to parse number token: {}", err.to_string()))
}

/// Converts input stream of chars into a die token if the following char is either F, for fudge, %
/// for percentile or any number of digits for any die.
fn parse_die_token(chars: &mut Peekable<Chars>) -> Result<Token, String> {
    let parse = |c| match c {
        'F' => Ok(Token::FudgeDie),
        '%' => Ok(Token::PercentileDie),
        '1'..='9' => Ok(Token::Die),
        c => Err(format!("Unexpected character: {}", c)),
    };

    chars
        .next_if_eq(&'d')
        .and(chars.next_if(|&c| c == 'F' || c == '%'))
        .or(chars.peek().copied())
        .ok_or("Unexpected end of input stream".to_string())
        .and_then(parse)
}

/// Converts a single [`char`] into a [`Token`].
fn parse_single_token(char: char, chars: &mut Peekable<Chars>) -> Result<Token, String> {
    match char {
        '+' => Ok(Token::Plus),
        '-' => Ok(Token::Minus),
        '*' => Ok(Token::Asterisk),
        '/' => Ok(Token::Slash),
        c => Err(format!("Unexpected character: {}", c)),
    }
    .and_then(|t| {
        chars.next();
        Ok(t)
    })
}

fn parse(char: char, chars: &mut Peekable<Chars>) -> Result<Token, String> {
    match char {
        '1'..='9' => parse_integer_token(chars),
        // TODO: needs to parsed as drop modifier too
        'd' => parse_die_token(chars),
        _ => parse_single_token(char, chars),
    }
}

#[test]
fn test_lex_valid() {
    let tests = [
        ("dF", vec![Token::FudgeDie]),
        ("d%", vec![Token::PercentileDie]),
        ("d20", vec![Token::Die, Token::Integer(20)]),
        (
            "d6*10",
            vec![
                Token::Die,
                Token::Integer(6),
                Token::Asterisk,
                Token::Integer(10),
            ],
        ),
        (
            "d6/10",
            vec![
                Token::Die,
                Token::Integer(6),
                Token::Slash,
                Token::Integer(10),
            ],
        ),
        (
            "2d20",
            vec![Token::Integer(2), Token::Die, Token::Integer(20)],
        ),
        (
            "d20-10",
            vec![
                Token::Die,
                Token::Integer(20),
                Token::Minus,
                Token::Integer(10),
            ],
        ),
        (
            "2d20+1d8",
            vec![
                Token::Integer(2),
                Token::Die,
                Token::Integer(20),
                Token::Plus,
                Token::Integer(1),
                Token::Die,
                Token::Integer(8),
            ],
        ),
        (
            "+-1234567890",
            vec![Token::Plus, Token::Minus, Token::Integer(1234567890)],
        ),
    ];

    tests.iter().for_each(|(input, expected)| {
        let actual = lex(input).unwrap();
        assert_eq!(actual, *expected, "for input `{:#?}`", input);
    })
}
