<!--
  Sync Impact Report
  ==================
  Version change: N/A → 1.0.0 (initial ratification)
  Modified principles: N/A (initial version)
  Added sections:
    - Core Principles (5 principles)
    - Technology Stack
    - Development Workflow
    - Governance
  Removed sections: N/A
  Templates requiring updates:
    - .specify/templates/plan-template.md ✅ compatible (Constitution Check is generic)
    - .specify/templates/spec-template.md ✅ compatible (no principle references)
    - .specify/templates/tasks-template.md ✅ compatible (no principle references)
  Follow-up TODOs: None
-->

# Calculator CLI Constitution

## Core Principles

### I. Library-First Architecture

All domain logic MUST reside in `lib.rs` (or library crate modules).
The binary crate (`main.rs`) MUST be a thin wrapper that handles
only I/O orchestration and user interaction. Rationale: this enforces
separation of concerns, enables independent unit testing of core
logic, and allows the library to be consumed by other crates.

- Every new feature MUST be implemented as a library function first.
- Library functions MUST NOT perform direct I/O (stdin/stdout);
  they accept data and return `Result` types.
- `main.rs` MUST NOT contain business logic beyond CLI dispatch.

### II. CLI Text Protocol

The CLI MUST follow a text-in/text-out protocol: user input via
stdin/args, results to stdout, errors to stderr.

- Normal output MUST go to stdout via `println!`.
- Error messages MUST go to stderr via `eprintln!`.
- The REPL loop MUST handle EOF and malformed input gracefully
  without panicking.
- Exit codes MUST be zero on success, non-zero on failure.

### III. Test-Driven Development

Tests MUST be written for all public library functions. New features
MUST have corresponding test cases before the implementation is
considered complete.

- Unit tests for pure logic MUST live in `lib.rs` under `#[cfg(test)]`.
- Integration tests exercising the public API MUST live in `tests/`.
- All tests MUST pass (`cargo test`) before any code is merged.
- Edge cases (empty input, division by zero, invalid tokens) MUST
  have explicit test coverage.

### IV. Correctness and Safety

The codebase MUST use Rust's type system and `Result`/`Error` types
to enforce correctness. Panics in user-facing code paths are
forbidden.

- All fallible operations MUST return `Result<T, Error>` using the
  project's custom `Error` enum.
- `unwrap()` and `expect()` MUST NOT appear in library code that
  handles user input. They are permitted only when the invariant is
  provably guaranteed by prior logic.
- New error variants MUST implement `Display` with a user-friendly
  message.
- Floating-point edge cases (division by zero, overflow) MUST be
  handled explicitly.

### V. Simplicity and Zero Dependencies

The project MUST remain dependency-free (no external crates) unless
a compelling, documented justification is provided for adding one.
Design decisions MUST favour the simplest correct solution.

- YAGNI: features MUST NOT be added speculatively.
- Abstractions MUST NOT be introduced for single-use cases.
- Code MUST be readable without extensive comments; prefer clear
  naming and small functions over documentation walls.
- New modules or files MUST be justified by a clear separation of
  concern, not organizational aesthetics.

## Technology Stack

- **Language**: Rust (edition 2021)
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Lint**: `cargo clippy` (all warnings MUST be resolved)
- **Format**: `cargo fmt` (all code MUST be formatted before merge)
- **External dependencies**: None. Adding a dependency requires an
  explicit justification recorded in the PR description.
- **Project type**: Library + CLI binary (single crate)

## Development Workflow

1. **Branch**: Create a feature branch from `main`.
2. **Implement**: Write tests first, then implementation in `lib.rs`.
   Wire into `main.rs` only after library tests pass.
3. **Validate**: Run `cargo fmt`, `cargo clippy`, and `cargo test`.
   All three MUST pass with zero warnings/failures.
4. **Review**: PRs MUST reference which Constitution principles were
   verified. Complexity additions MUST be justified.
5. **Merge**: Squash-merge into `main` with a descriptive commit
   message.

## Governance

This Constitution is the authoritative source of project standards.
It supersedes ad-hoc conventions and informal agreements.

- **Amendments**: Any change to this document MUST be reviewed,
  justified, and merged via a dedicated PR. The version MUST be
  incremented per semantic versioning (see below).
- **Versioning**:
  - MAJOR: Removal or incompatible redefinition of a principle.
  - MINOR: New principle or materially expanded guidance.
  - PATCH: Clarifications, wording, or typo fixes.
- **Compliance**: All PRs and code reviews MUST verify adherence to
  the principles listed above. Violations MUST be resolved before
  merge.
- **Guidance**: Runtime development guidance (e.g., quickstart docs,
  onboarding notes) SHOULD reference this Constitution but MUST NOT
  contradict it.

**Version**: 1.0.0 | **Ratified**: 2026-03-20 | **Last Amended**: 2026-03-20
