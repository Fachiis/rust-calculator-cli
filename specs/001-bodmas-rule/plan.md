# Implementation Plan: BODMAS Rule Support

**Branch**: `001-bodmas-rule` | **Date**: 2026-03-20 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-bodmas-rule/spec.md`

## Summary

Complete BODMAS compliance in the calculator CLI by adding parenthesis grouping (Brackets) and the `^` exponentiation operator (Orders) to the existing Shunting-yard/RPN evaluation pipeline. The existing `+`, `-`, `*`, `/` behaviour is preserved unchanged. Non-finite results (NaN, Infinity) produce descriptive errors.

## Technical Context

**Language/Version**: Rust, edition 2021  
**Primary Dependencies**: None (zero external crates)  
**Storage**: N/A  
**Testing**: `cargo test` (unit tests in `src/lib.rs`, integration tests in `tests/unit.rs`)  
**Target Platform**: macOS / any platform supported by `cargo`  
**Project Type**: Library + CLI binary (single crate)  
**Performance Goals**: N/A (single-user interactive REPL)  
**Constraints**: Zero external dependencies; all logic in `lib.rs`  
**Scale/Scope**: ~210 LOC library, ~55 LOC binary

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Rationale |
|-----------|--------|-----------|
| I. Library-First Architecture | PASS | All new parsing, precedence, and evaluation logic will be added to `src/lib.rs`. `main.rs` requires zero changes. |
| II. CLI Text Protocol | PASS | Input/output contract unchanged. New tokens (`(`, `)`, `^`) are parsed from existing space-separated stdin input. Errors go to stderr. |
| III. Test-Driven Development | PASS | Plan includes unit tests for bracket parsing, exponentiation, precedence, and RPN conversion. Integration tests in `tests/`. |
| IV. Correctness and Safety | PASS | New error variants (`MismatchedBracket`, `EmptyBrackets`, `NotFinite`) use the custom `Error` enum with `Display`. No new panics introduced. Existing `panic!` in `apply_operator` will be replaced with `Err`. |
| V. Simplicity and Zero Dependencies | PASS | No new crates. Extends existing Shunting-yard algorithm (standard textbook approach for bracket/precedence handling). No new files or modules — changes stay in `lib.rs`. |

**GATE RESULT**: PASS — all 5 principles satisfied. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-bodmas-rule/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (CLI interface contract)
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # All domain logic (parsing, tokens, RPN, evaluation)
└── main.rs              # Thin REPL wrapper (no changes needed)

tests/
└── unit.rs              # Integration tests for public API
```

**Structure Decision**: Single crate, single library file. The existing structure is sufficient — no new files or modules are justified given the scope of changes (extending `Token` enum, `precedence()`, `to_rpn()`, `parse_expression()`, `apply_operator()`, and adding error variants).

## Constitution Check — Post-Design Re-evaluation

*Re-checked after Phase 1 design artifacts were produced.*

| Principle | Status | Post-Design Rationale |
|-----------|--------|----------------------|
| I. Library-First Architecture | PASS | data-model.md confirms all changes are in `Token`/`Error` enums and private functions within `lib.rs`. Public API signatures unchanged. |
| II. CLI Text Protocol | PASS | contracts/cli-interface.md defines space-separated token grammar. New tokens follow existing pattern. Errors to stderr. |
| III. Test-Driven Development | PASS | data-model.md validation rules table maps each rule to a testable function. quickstart.md documents `cargo test`. |
| IV. Correctness and Safety | PASS | research.md R5 replaces `panic!` with `Err`. R4 adds `is_finite()` check. Three new error variants cover all failure modes. |
| V. Simplicity and Zero Dependencies | PASS | No new crates, files, or modules. Standard Shunting-yard extension only. |

**POST-DESIGN GATE RESULT**: PASS — all 5 principles satisfied.

## Complexity Tracking

No violations detected. Table intentionally left empty.

## Generated Artifacts

| Artifact | Path | Phase |
|----------|------|-------|
| Research | `specs/001-bodmas-rule/research.md` | Phase 0 |
| Data Model | `specs/001-bodmas-rule/data-model.md` | Phase 1 |
| CLI Contract | `specs/001-bodmas-rule/contracts/cli-interface.md` | Phase 1 |
| Quickstart | `specs/001-bodmas-rule/quickstart.md` | Phase 1 |
| Agent Context | `.github/agents/copilot-instructions.md` | Phase 1 |
