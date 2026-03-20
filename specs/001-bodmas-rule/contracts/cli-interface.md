# CLI Interface Contract: calculator-cli

**Feature**: 001-bodmas-rule
**Date**: 2026-03-20

## Input Contract

**Interface**: Interactive REPL via stdin/stdout.

### Token Grammar (space-separated)

```text
expression := token ( SPACE token )*
token      := number | operator | "(" | ")"
number     := [0-9]+ ( "." [0-9]+ )?
operator   := "+" | "-" | "*" | "/" | "^"
SPACE      := " "+
```

**Examples**:

| Input                       | Expected Output |
| --------------------------- | --------------- |
| `2 + 3`                     | `5`             |
| `( 2 + 3 ) * 4`             | `20`            |
| `2 ^ 3`                     | `8`             |
| `2 ^ 3 ^ 2`                 | `512`           |
| `( 1 + 2 ) ^ ( 1 + 1 )`     | `9`             |
| `( ( 2 + 3 ) * ( 4 - 1 ) )` | `15`            |

### Special Commands

| Command       | Behaviour                          |
| ------------- | ---------------------------------- |
| `help`        | Print help documentation to stdout |
| `quit` or `q` | Exit the REPL                      |

## Output Contract

### Success

Result printed to stdout as a single line, formatted by Rust's default `f64` `Display` trait.

```text
> ( 2 + 3 ) * 4
20
```

### Errors

Error messages printed to stderr, prefixed with `Error:`.

| Error Condition       | Message                            |
| --------------------- | ---------------------------------- |
| Mismatched brackets   | `Error: Mismatched brackets`       |
| Empty brackets `( )`  | `Error: Empty brackets`            |
| Non-finite result     | `Error: Result is not finite`      |
| Division by zero      | `Error: Division by zero`          |
| Invalid number        | `Error: Invalid number: {token}`   |
| Invalid operator      | `Error: Invalid operator: {token}` |
| Empty expression      | `Error: Empty expression`          |
| Consecutive operators | `Error: Consecutive operators`     |
| Too many operators    | `Error: Too many operators`        |
| Invalid expression    | `Error: Invalid expression`        |

## Public Library API

```rust
/// Parse a whitespace-separated expression into tokens.
pub fn parse_expression(input: &[&str]) -> Result<Vec<Token>, Error>;

/// Evaluate a token sequence and return the numeric result.
pub fn evaluate_expression(tokens: Vec<Token>) -> Result<f64, Error>;

/// Print help documentation.
pub fn print_help_doc();
```

These three functions form the stable public API. Internal functions (`to_rpn`, `evaluate_rpn`, `apply_operator`, `precedence`) remain private.

## Backward Compatibility

All existing expressions that use `+`, `-`, `*`, `/` produce identical results. The `(`, `)`, and `^` tokens are new additions — they were previously rejected as invalid operators.
