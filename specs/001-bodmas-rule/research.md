# Research: BODMAS Rule Support

**Feature**: 001-bodmas-rule
**Date**: 2026-03-20

## R1: Extending Shunting-yard for Parentheses

**Context**: The existing `to_rpn()` in `lib.rs` implements the Shunting-yard algorithm for `+`, `-`, `*`, `/`. Brackets need to be added.

**Decision**: Extend `to_rpn()` with standard parenthesis handling from Dijkstra's Shunting-yard algorithm.

**Rationale**: The Shunting-yard algorithm natively supports parentheses with two simple rules:

1. On `(` — push to operator stack.
2. On `)` — pop operators to output until `(` is found; discard both parens.

This is the textbook approach and adds minimal code (~10-15 lines) to the existing `to_rpn()` function.

**Alternatives considered**:

- Recursive descent parser: More complex, would require rewriting the entire parsing pipeline. Rejected because the Shunting-yard extension is simpler and the existing implementation already works well.
- Separate pre-processing pass to eliminate brackets: Would require expression tree construction. Rejected as it introduces unnecessary abstraction.

## R2: Right-Associative Exponentiation (`^`)

**Context**: The `^` operator must be right-associative (i.e., `2 ^ 3 ^ 2` = `2 ^ 9` = 512, not `8 ^ 2` = 64). Current `precedence()` and `to_rpn()` assume all operators are left-associative.

**Decision**: Assign `^` precedence 3 (above `*`/`/` at 2) and modify the Shunting-yard comparison to use strict `>` instead of `>=` when the current operator is right-associative.

**Rationale**: In the standard Shunting-yard algorithm, left-associative operators pop the stack when the stack operator has equal-or-higher precedence (`>=`). Right-associative operators pop only when the stack operator has strictly higher precedence (`>`). This is the minimal change needed:

- Add a helper function or match arm that returns associativity for `^`.
- Change the comparison in `to_rpn()` from `>=` to `>` when the incoming operator is `^`.

**Alternatives considered**:

- Encode associativity in the precedence value (e.g., use half-integers): Clever but obscure. Rejected for readability.
- Separate right-associative operator stack: Overly complex. Rejected.

## R3: Token Enum Extension

**Context**: Current `Token` enum has `Number(f64)` and `Operator(char)`. Parentheses need to be represented.

**Decision**: Add `LeftParen` and `RightParen` variants to the `Token` enum.

**Rationale**: Dedicated variants are cleaner than encoding `(` and `)` as `Operator('(')` because:

1. Parentheses are not operators — they don't go through `apply_operator()`.
2. Pattern matching is clearer with distinct variants.
3. `precedence()` and `apply_operator()` don't need to special-case parens.

**Alternatives considered**:

- Encode as `Operator('(')` / `Operator(')')`: Mixes concerns. Would require guards in `apply_operator()` and `precedence()`. Rejected.
- Create a separate `Delimiter` enum: Over-abstraction for two values. Rejected.

## R4: Error Handling for Non-Finite Results

**Context**: Per spec FR-013, exponentiation like `999999 ^ 999999` produces `f64::INFINITY`, and `0 ^ -1` division scenarios may produce `NaN`. These must be caught and reported as errors.

**Decision**: Add a `NotFinite` variant to the `Error` enum. Check the result of `apply_operator()` for `^` (and optionally all operators) using `f64::is_finite()`. Return the error immediately if the result is not finite.

**Rationale**: Rust's `f64` arithmetic does not panic on overflow — it produces `INFINITY` or `NaN` silently. A single `is_finite()` check after each `^` application is sufficient. Checking after all operations (not just `^`) is a minimal cost for stronger correctness guarantees.

**Alternatives considered**:

- Check only at the end of evaluation: Would allow intermediate non-finite values to propagate, potentially masking the original cause. Rejected.
- Use a custom numeric type that traps on overflow: Massive over-engineering for a CLI calculator. Rejected.

## R5: Replacing panic! in apply_operator

**Context**: The existing `apply_operator()` contains `_ => panic!("Unknown operator")`. This violates Constitution Principle IV (Correctness and Safety). Since we're modifying this function to add `^`, we should fix this.

**Decision**: Replace the `panic!` with `Err(Error::InvalidOperator(op.to_string()))` where `op` is the unrecognised character.

**Rationale**: The `Error::InvalidOperator` variant already exists in the enum. Returning it instead of panicking aligns with Principle IV and is a one-line fix while we're already modifying the function. The function signature changes from returning `f64` to `Result<f64, Error>`, which requires updating callers in `evaluate_rpn()`.

**Alternatives considered**:

- Leave the panic and only add `^`: Feature would work, but a known safety violation would persist in code we're actively modifying. Rejected per Constitution Principle IV.

## R6: Empty Brackets and Edge Cases

**Context**: Spec edge case EC-005 requires that `( )` produces an error. EC-006/EC-007 require mismatched brackets to error.

**Decision**:

- **Empty brackets**: Detect in `to_rpn()` — if `)` is encountered and the previous token was `(` (or the stack top is `(` with nothing output between), emit `Error::EmptyBrackets`.
- **Mismatched brackets**: Standard Shunting-yard handling — if `)` is encountered and no `(` is found on the stack, emit `Error::MismatchedBracket`. After processing all tokens, if `(` remains on the stack, emit the same error.

**Rationale**: These are standard edge cases for any parenthesis-supporting expression parser. The Shunting-yard algorithm naturally detects mismatched brackets; we just need to return errors instead of silently producing invalid RPN.

**Alternatives considered**: None — this is the only correct approach for Shunting-yard bracket validation.
