# @droll/wasm

`@droll/wasm` parses the dice notation by utilizing a [operator-precedence
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

## Usage

To use `@droll/wasm` simply import the `roll` function. The `roll` function
takes [dice notation](https://en.wikipedia.org/wiki/Dice_notation) as input.

```ts
import { roll } from "@droll/wasm";

const result: number = roll("1d20+10");

console.log(result); // e.g. 27
```

To run the `wasm` code with nodejs you need to call node with an experimental
flag:

```
NODE_NO_WARNINGS=1 node --experimental-wasm-modules calls_wasm_roll_function.js
```

### Standard dice notation

The simplest dice notation is called [_standard dice
notation_](https://en.wikipedia.org/wiki/Dice_notation) and it supports simple
dice rolls like `d6`, `2d20` and also additive operations like `2d20+10-2`.

To calculate the roll result, simply call the `roll` function with the
desired dice notation:

```ts
import { roll } from "@droll/wasm";

roll("d6"); // e.g. 4
roll("1d20+10"); // e.g. 27
roll("d100"); // e.g. 78
```
