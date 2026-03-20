# Tasks: BODMAS Rule Support

**Input**: Design documents from `/specs/001-bodmas-rule/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-interface.md, quickstart.md

**Tests**: Included — Constitution Principle III (TDD) mandates test-driven development, and SC-003 requires every functional requirement to have at least one automated test.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Extend core types and fix function signatures that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T001 Add LeftParen and RightParen variants to Token enum in src/lib.rs
- [X] T002 Add MismatchedBracket, EmptyBrackets, and NotFinite variants with Display messages to Error enum in src/lib.rs
- [X] T003 Change apply_operator() to return Result<f64, Error>, replace panic!("Unknown operator") with Err(Error::InvalidOperator(...)) in src/lib.rs
- [X] T004 Update evaluate_rpn() to propagate Result from apply_operator() using ? operator in src/lib.rs
- [X] T005 Fix existing test_apply_operator unit test to expect Result<f64, Error> return type in src/lib.rs

**Checkpoint**: Core types extended, apply_operator returns Result — user story implementation can now begin

---

## Phase 3: User Story 1 — Bracket Grouping (Priority: P1) 🎯 MVP

**Goal**: Users can group sub-expressions with `( )` to override default operator precedence

**Independent Test**: Enter `( 2 + 3 ) * 4` and verify result is `20` (vs `14` without brackets)

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T006 [US1] Write unit tests for parse_expression() with ( and ) tokens, and to_rpn() with bracket handling in src/lib.rs
- [X] T007 [P] [US1] Write integration tests for bracket evaluation (( 2 + 3 ) * 4 = 20, 10 / ( 2 + 3 ) = 2, ( ( 1 + 2 ) * 3 ) = 9) and bracket error cases (mismatched, empty) in tests/unit.rs

### Implementation for User Story 1

- [X] T008 [US1] Update parse_expression() to recognize ( as LeftParen and ) as RightParen tokens in src/lib.rs
- [X] T009 [US1] Extend to_rpn() with parenthesis handling: push LeftParen to operator stack, pop operators to output until LeftParen on RightParen in src/lib.rs
- [X] T010 [US1] Add bracket validation in to_rpn(): mismatched bracket detection (unmatched ( on stack or ) with no matching () and empty bracket detection (( ) with nothing inside) in src/lib.rs

**Checkpoint**: Bracket grouping fully functional — `( 2 + 3 ) * 4` = `20`, error on mismatched/empty brackets

---

## Phase 4: User Story 2 — Exponentiation (Priority: P2)

**Goal**: Users can use `^` operator for exponentiation with correct precedence and right-associativity

**Independent Test**: Enter `2 ^ 3 ^ 2` and verify result is `512` (right-associative: 2^(3^2) = 2^9)

### Tests for User Story 2

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T011 [US2] Write unit tests for precedence('^') = 3, is_right_associative('^'), apply_operator with ^, and to_rpn() right-associativity in src/lib.rs
- [X] T012 [P] [US2] Write integration tests for exponentiation expressions (2 ^ 3 = 8, 3 + 2 ^ 3 = 11, 2 ^ 3 ^ 2 = 512) and edge cases (0 ^ 0 = 1, non-finite errors) in tests/unit.rs

### Implementation for User Story 2

- [X] T013 [US2] Add ^ as valid operator in parse_expression() and add ^ with precedence 3 to precedence() in src/lib.rs
- [X] T014 [US2] Add is_right_associative() helper function and update to_rpn() operator comparison to use strict > for right-associative operators in src/lib.rs
- [X] T015 [US2] Add ^ case to apply_operator() using f64::powf() in src/lib.rs
- [X] T016 [US2] Add is_finite() check after apply_operator() result in evaluate_rpn(), returning Err(Error::NotFinite) for Infinity/NaN in src/lib.rs

**Checkpoint**: Exponentiation fully functional — `2 ^ 3 ^ 2` = `512`, `0 ^ 0` = `1`, non-finite results error

---

## Phase 5: User Story 3 — Combined BODMAS Expressions (Priority: P3)

**Goal**: Complex expressions combining brackets, exponents, and basic arithmetic evaluate correctly per full BODMAS order

**Independent Test**: Enter `( 2 + 3 ) ^ 2` and verify result is `25`

**Dependencies**: Requires US1 (brackets) and US2 (exponentiation) to be complete

### Tests for User Story 3

- [X] T017 [US3] Write integration tests for combined bracket + exponent expressions (( 2 + 3 ) ^ 2 = 25, 2 ^ ( 1 + 2 ) * 3 = 24, ( 3 + 2 ) * ( 2 ^ 3 - 1 ) = 35) in tests/unit.rs

### Validation for User Story 3

- [X] T018 [US3] Run cargo test and verify all spec acceptance scenarios from US1, US2, and US3 pass

**Checkpoint**: Full BODMAS compliance verified — all user stories pass independently and together

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation updates and final validation across all stories

- [X] T019 Update print_help_doc() with bracket and exponent usage examples per FR-012 in src/lib.rs
- [X] T020 Run cargo clippy and cargo fmt, fix any warnings or formatting issues
- [X] T021 Run full test suite (cargo test) and validate quickstart.md scenarios manually in REPL

---

## Dependencies & Execution Order

### Phase Dependencies

- **Foundational (Phase 2)**: No dependencies — start immediately. BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion
- **User Story 2 (Phase 4)**: Depends on Phase 2 completion
- **User Story 3 (Phase 5)**: Depends on Phase 3 AND Phase 4 completion
- **Polish (Phase 6)**: Depends on Phase 5 completion

### User Story Dependencies

- **User Story 1 (P1)**: Independent of US2 — can start after Foundational
- **User Story 2 (P2)**: Independent of US1 — can start after Foundational
- **User Story 3 (P3)**: Depends on BOTH US1 and US2 — validates combined behaviour

### Within Each User Story

- Tests MUST be written and FAIL before implementation (TDD per Constitution Principle III)
- Parse changes before algorithm changes
- Algorithm changes before validation/error handling
- Story complete before moving to next priority

### Parallel Opportunities

- **Phase 2**: All tasks are sequential (same file, dependent changes)
- **Phase 3**: T006 + T007 can run in parallel (src/lib.rs unit tests + tests/unit.rs integration tests)
- **Phase 4**: T011 + T012 can run in parallel (src/lib.rs unit tests + tests/unit.rs integration tests)
- **Cross-story**: US1 (Phase 3) and US2 (Phase 4) can run in parallel after Foundational completes

---

## Parallel Example: User Story 1

```bash
# Launch tests for US1 in parallel (different files):
Task T006: "Unit tests for bracket parsing in src/lib.rs"
Task T007: "Integration tests for bracket evaluation in tests/unit.rs"

# Then sequential implementation in src/lib.rs:
Task T008: "Update parse_expression() for ( and )"
Task T009: "Extend to_rpn() with parenthesis handling"
Task T010: "Add bracket validation (mismatched, empty)"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 2: Foundational (type extensions, apply_operator fix)
2. Complete Phase 3: User Story 1 — Bracket Grouping
3. **STOP and VALIDATE**: `cargo test` — brackets work, all existing tests still pass
4. Deploy/demo if ready — users can group expressions with `( )`

### Incremental Delivery

1. Foundational → Core types ready
2. Add User Story 1 → Test independently → Brackets work (MVP!)
3. Add User Story 2 → Test independently → Exponentiation works
4. Add User Story 3 → Test combined → Full BODMAS validated
5. Polish → Help text, linting, final validation

### Parallel Strategy

After Foundational completes:
- Worker A: User Story 1 (Brackets) in src/lib.rs
- Worker B: User Story 2 (Exponentiation) — NOTE: same file as US1, so true parallel requires careful merge
- Recommended: Sequential P1 → P2 for single-developer workflow (avoids merge conflicts in src/lib.rs)

---

## Notes

- All source changes are in a single file (src/lib.rs) — parallel within stories is limited
- [P] tasks are test tasks in different files (src/lib.rs #[cfg(test)] vs tests/unit.rs)
- Each user story is independently testable once its phase completes
- Commit after each task or logical group
- Stop at any checkpoint to validate the story independently
- No new files or crates — all changes extend existing code per Constitution Principle V
