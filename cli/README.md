# droll-cli

`droll-cli` parses the dice notation by utilizing a [operator-precedence
parser](https://en.wikipedia.org/wiki/Operator-precedence_parser) according to
the following grammar rules (The grammar is
[EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
format).

```
<expr> ::= <roll-expr>
         | <expr> '+' <expr>
         | <expr> '-' <expr>

<roll-expr> ::= <primary>
              | <expr> 'd' <expr>

<primary> ::= <number>
            | '+' <primary>
            | '-' <primary>
            | 'd' <expr>

<number> ::= <non-zero-digit> { <digit> }

<non-zero-digit> ::= '1' .. '9'

<digit> ::= '0' .. '9'
```

## Installation

To install `droll-cli`, you can use the Rust package manager, `Cargo`, with the following command:

```
cargo install droll
```

## Usage

After installing `droll-cli`, you can use it by calling it from the shell with the desired dice notation:

```
droll <dice_notation>
```

Replace <dice_notation> with actual [dice notation](https://en.wikipedia.org/wiki/Dice_notation) like `1d20+10`.

### Standard dice notation

The simplest dice notation is called [_standard dice
notation_](https://en.wikipedia.org/wiki/Dice_notation) and it supports simple
dice rolls like `d6`, `2d20` and also additive operations like `2d20+10-2`.

To calculate the roll result, simply call the `droll` with the desired dice
notation:

```
droll 1d20+10
27
```
