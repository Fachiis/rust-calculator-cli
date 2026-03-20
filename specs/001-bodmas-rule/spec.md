# Feature Specification: BODMAS Rule Support

**Feature Branch**: `001-bodmas-rule`
**Created**: 2026-03-20
**Status**: Draft
**Input**: User description: "I want to add a BODMAS rule to the calculator application."

## Overview

The calculator currently evaluates arithmetic expressions with correct precedence for division, multiplication, addition, and subtraction (the DMAS portion of BODMAS). This feature completes BODMAS compliance by adding support for **Brackets** (parentheses) and **Orders** (exponentiation), so users can write expressions like `( 2 + 3 ) * 4` and `2 ^ 3`.

## Clarifications

### Session 2026-03-20

- Q: What should the calculator do when exponentiation produces a non-finite result (NaN or Infinity)? → A: Return a descriptive error (e.g., "Result is not a finite number")
- Q: Should the calculator enforce a maximum bracket nesting depth? → A: No artificial limit; rely on available memory (single-user CLI, no server-side resource concern)

## Assumptions

- Brackets use the standard `(` and `)` characters.
- The exponentiation operator uses the `^` character (caret), which is the most common convention for text-based calculators.
- Exponentiation is right-associative: `2 ^ 3 ^ 2` evaluates as `2 ^ (3 ^ 2) = 512`, matching mathematical convention.
- Exponentiation has higher precedence than multiplication/division.
- Brackets and operators remain space-separated tokens in user input (matching existing input convention), e.g., `( 2 + 3 ) * 4` rather than `(2+3)*4`.
- Existing behaviour for `+`, `-`, `*`, `/` MUST NOT change.
- Nested brackets are supported to arbitrary depth, limited only by available memory.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Bracket Grouping (Priority: P1)

A user enters an expression with parentheses to override default operator precedence, and the calculator respects the grouping.

**Why this priority**: Brackets are the most impactful missing BODMAS component. Without them, users cannot control evaluation order, which is a fundamental arithmetic capability.

**Independent Test**: Can be fully tested by entering expressions with brackets and verifying the result differs from the non-bracketed equivalent, e.g., `( 2 + 3 ) * 4` = 20 versus `2 + 3 * 4` = 14.

**Acceptance Scenarios**:

1. **Given** the calculator is running, **When** the user enters `( 2 + 3 ) * 4`, **Then** the result is `20`.
2. **Given** the calculator is running, **When** the user enters `10 / ( 2 + 3 )`, **Then** the result is `2`.
3. **Given** the calculator is running, **When** the user enters `( 1 + 2 ) * ( 3 + 4 )`, **Then** the result is `21`.
4. **Given** the calculator is running, **When** the user enters `( ( 1 + 2 ) * 3 )`, **Then** the result is `9` (nested brackets).

---

### User Story 2 - Exponentiation (Priority: P2)

A user enters an expression using the `^` operator to raise a number to a power, and the calculator evaluates it with correct precedence and right-associativity.

**Why this priority**: Exponentiation completes the "Orders" component of BODMAS and is the second most impactful gap. However, many calculator workflows function well with just brackets and basic arithmetic.

**Independent Test**: Can be fully tested by entering power expressions and verifying results, e.g., `2 ^ 3` = 8.

**Acceptance Scenarios**:

1. **Given** the calculator is running, **When** the user enters `2 ^ 3`, **Then** the result is `8`.
2. **Given** the calculator is running, **When** the user enters `3 + 2 ^ 3`, **Then** the result is `11` (exponent evaluated before addition).
3. **Given** the calculator is running, **When** the user enters `2 ^ 3 ^ 2`, **Then** the result is `512` (right-associative: 2^(3^2) = 2^9).
4. **Given** the calculator is running, **When** the user enters `2 * 3 ^ 2`, **Then** the result is `18` (exponent before multiplication).

---

### User Story 3 - Combined BODMAS Expressions (Priority: P3)

A user enters a complex expression combining brackets, exponents, and basic arithmetic, and the calculator evaluates it following full BODMAS order.

**Why this priority**: This validates end-to-end correctness when all BODMAS components interact together. Depends on US1 and US2 being complete.

**Independent Test**: Can be fully tested by entering compound expressions and comparing results against known mathematical answers.

**Acceptance Scenarios**:

1. **Given** the calculator is running, **When** the user enters `( 2 + 3 ) ^ 2`, **Then** the result is `25`.
2. **Given** the calculator is running, **When** the user enters `2 ^ ( 1 + 2 ) * 3`, **Then** the result is `24` (brackets first → exponent → multiplication).
3. **Given** the calculator is running, **When** the user enters `( 3 + 2 ) * ( 2 ^ 3 - 1 )`, **Then** the result is `35`.

---

### Edge Cases

- What happens when brackets are mismatched (e.g., `( 2 + 3` or `2 + 3 )`)? The system MUST return a clear error message indicating the mismatch.
- What happens with empty brackets (e.g., `( )`)? The system MUST return an error for an empty sub-expression.
- What happens with `0 ^ 0`? The system MUST return `1` (following the standard mathematical convention used by IEEE 754).
- What happens with a negative exponent (e.g., `2 ^ -3`)? The system MUST handle this if the user enters it as `2 ^ ( 0 - 3 )`. Direct negative numbers as tokens (e.g., `-3` without preceding operator) are out of scope for this feature.
- What happens with a fractional exponent (e.g., `4 ^ 0.5`)? The system MUST evaluate it correctly (result: `2`).
- What happens with deeply nested brackets (e.g., `( ( ( 1 + 2 ) ) )`)? The system MUST evaluate correctly.
- What happens when exponentiation overflows (e.g., `999 ^ 999`)? The system MUST return an error indicating the result is not finite.
- What happens when exponentiation produces NaN (e.g., `( 0 - 1 ) ^ 0.5`)? The system MUST return an error indicating the result is not a finite number.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST recognise `(` and `)` as valid bracket tokens in expressions.
- **FR-002**: System MUST evaluate sub-expressions within brackets before applying operators outside the brackets.
- **FR-003**: System MUST support nested brackets to arbitrary depth.
- **FR-004**: System MUST return a clear, user-friendly error when brackets are mismatched (unclosed opening bracket or unexpected closing bracket).
- **FR-005**: System MUST return an error when brackets contain no valid sub-expression (empty brackets).
- **FR-006**: System MUST recognise `^` as the exponentiation operator.
- **FR-007**: System MUST evaluate `^` with higher precedence than `*` and `/`.
- **FR-008**: System MUST evaluate `^` as right-associative (e.g., `2 ^ 3 ^ 2` = `2 ^ (3 ^ 2)`).
- **FR-009**: System MUST evaluate `0 ^ 0` as `1`.
- **FR-010**: System MUST support fractional exponents (e.g., `4 ^ 0.5` = `2`).
- **FR-011**: All existing expressions without brackets or `^` MUST continue to produce identical results (backward compatibility).
- **FR-012**: The help text MUST be updated to show bracket and exponent usage examples.
- **FR-013**: System MUST return a clear error when an exponentiation result is not a finite number (NaN or Infinity), e.g., `( 0 - 1 ) ^ 0.5` or `999 ^ 999`.

### Key Entities

- **Token**: Represents a parsed element of an expression. Currently has `Number` and `Operator` variants. Must be extended with bracket variants (open parenthesis, close parenthesis).
- **Operator Precedence**: The ordering that determines evaluation sequence. Currently two levels (add/sub = 1, mul/div = 2). Must add a third level for exponentiation.
- **Expression**: The full user-entered calculation. Must now support grouping via brackets and the `^` operator.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All acceptance scenarios from User Stories 1–3 pass when tested manually in the REPL.
- **SC-002**: 100% of existing tests continue to pass without modification (backward compatibility).
- **SC-003**: Every new functional requirement (FR-001 through FR-012) has at least one corresponding automated test.
- **SC-004**: Users can evaluate any valid BODMAS expression in a single input line and receive the correct result.
- **SC-005**: Users receive a clear, actionable error message within 1 second for any malformed expression involving brackets or exponents.
