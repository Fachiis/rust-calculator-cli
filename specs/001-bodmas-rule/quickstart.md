# Quickstart: BODMAS Rule Support

**Feature**: 001-bodmas-rule
**Date**: 2026-03-20

## Prerequisites

- Rust toolchain (edition 2021)
- `cargo` available in PATH

## Build & Run

```bash
# Build
cargo build

# Run the REPL
cargo run
```

## Test

```bash
# Run all tests (unit + integration)
cargo test

# Run with output visible
cargo test -- --nocapture
```

## Lint

```bash
cargo clippy
cargo fmt --check
```

## Try It Out

After `cargo run`, enter expressions at the `>` prompt:

```text
> 2 + 3
5

> ( 2 + 3 ) * 4
20

> 2 ^ 3
8

> 2 ^ 3 ^ 2
512

> ( ( 1 + 2 ) * ( 3 + 4 ) )
21

> help
[prints help documentation]

> quit
```

## Key Files

| File            | Purpose                                                  |
| --------------- | -------------------------------------------------------- |
| `src/lib.rs`    | All domain logic: Token, Error, parsing, RPN, evaluation |
| `src/main.rs`   | Thin REPL wrapper (stdin/stdout loop)                    |
| `tests/unit.rs` | Integration tests for public API                         |
| `Cargo.toml`    | Crate metadata (zero dependencies)                       |
