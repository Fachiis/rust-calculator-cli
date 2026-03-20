# Data Model: BODMAS Rule Support

**Feature**: 001-bodmas-rule
**Date**: 2026-03-20

## Entities

### Token (enum) — Extended

Current definition in `src/lib.rs`:

```rust
enum Token {
    Number(f64),
    Operator(char),
}
```

Extended definition:

```rust
enum Token {
    Number(f64),
    Operator(char),    // '+', '-', '*', '/', '^'
    LeftParen,         // NEW
    RightParen,        // NEW
}
```

| Variant          | Fields                         | Source               | Notes                                                       |
| ---------------- | ------------------------------ | -------------------- | ----------------------------------------------------------- |
| `Number(f64)`    | Parsed floating-point value    | Existing             | Unchanged                                                   |
| `Operator(char)` | One of `+`, `-`, `*`, `/`, `^` | Existing + `^` added | `^` is right-associative                                    |
| `LeftParen`      | None                           | NEW                  | Produced by `parse_expression()` when `(` token encountered |
| `RightParen`     | None                           | NEW                  | Produced by `parse_expression()` when `)` token encountered |

### Error (enum) — Extended

Current definition in `src/lib.rs`:

```rust
pub enum Error {
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidExpression,
    ConsecutiveOperators,
    DivisionByZero,
    TooManyOperators,
    EmptyExpression,
}
```

Extended definition:

```rust
pub enum Error {
    InvalidNumber(String),
    InvalidOperator(String),
    InvalidExpression,
    ConsecutiveOperators,
    DivisionByZero,
    TooManyOperators,
    EmptyExpression,
    MismatchedBracket,   // NEW — FR-004, FR-005
    EmptyBrackets,       // NEW — FR-006, EC-005
    NotFinite,           // NEW — FR-013
}
```

| Variant             | Display Message                 | Trigger                                 | Spec Ref       |
| ------------------- | ------------------------------- | --------------------------------------- | -------------- |
| `MismatchedBracket` | `"Error: Mismatched brackets"`  | Unmatched `(` or `)` in expression      | FR-004, FR-005 |
| `EmptyBrackets`     | `"Error: Empty brackets"`       | `( )` with no expression inside         | FR-006, EC-005 |
| `NotFinite`         | `"Error: Result is not finite"` | Exponentiation produces Infinity or NaN | FR-013         |

### Precedence Table

| Operator | Precedence (i32) | Associativity | Source               |
| -------- | ---------------- | ------------- | -------------------- |
| `+`      | 1                | Left          | Existing             |
| `-`      | 1                | Left          | Existing             |
| `*`      | 2                | Left          | Existing             |
| `/`      | 2                | Left          | Existing             |
| `^`      | 3                | Right         | NEW — FR-009, FR-010 |

## Relationships

```text
Input string
  ──parse_expression()──▶ Vec<Token>
                            │
                       to_rpn()
                            │
                            ▼
                        Vec<Token>  (RPN-ordered, no parens)
                            │
                      evaluate_rpn()
                            │
                            ▼
                         f64 result ──is_finite()?──▶ Ok(f64) or Err(NotFinite)
```

- `parse_expression()` produces `Token` sequence including `LeftParen`/`RightParen`.
- `to_rpn()` consumes parens during Shunting-yard conversion; output never contains `LeftParen`/`RightParen`.
- `evaluate_rpn()` operates on `Number` and `Operator` tokens only.
- `apply_operator()` handles `^` and returns `Result<f64, Error>` (changed from `f64`).

## Validation Rules

| Rule                                                | Location                                          | Spec Ref |
| --------------------------------------------------- | ------------------------------------------------- | -------- |
| `(` must have matching `)`                          | `to_rpn()`                                        | FR-004   |
| `)` must have matching `(`                          | `to_rpn()`                                        | FR-005   |
| `( )` with no content is an error                   | `to_rpn()`                                        | FR-006   |
| `^` right operand applied before left (right-assoc) | `to_rpn()` precedence comparison                  | FR-010   |
| `0 ^ 0` evaluates to `1.0`                          | `apply_operator()` (Rust `f64::powf` returns 1.0) | FR-011   |
| Non-finite results are errors                       | `evaluate_rpn()` after `apply_operator()`         | FR-013   |
| Division by zero already handled                    | `apply_operator()`                                | Existing |

## State Transitions

N/A — the calculator is stateless. Each expression is parsed and evaluated independently.
