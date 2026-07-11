# 291x-787 Loop Simple While Extractor Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/facts/extractors/loop_simple_while.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory split `loop_simple_while.rs` into:

- live shared helper: `validate_loop_condition_structure`
- dead staged extractor shelf:
  - `LoopSimpleWhileParts`
  - `extract_loop_simple_while_parts`
  - `has_simple_step_pattern`
  - `has_control_flow_statement`

Repository search found no non-test callers for the extractor path, while
`validate_loop_condition_structure` remains live through `if_phi_join.rs`.

## Decision

Retire the dead extractor shelf and keep the file only as the shared
condition-shape helper owner. Replace self-tests for the dead extractor with
tests for the still-live helper.

## Landed

- Removed the dead `loop_simple_while` extractor surface and its helper
  internals.
- Removed the file-level dead-code hold.
- Narrowed the file docs to the live helper role.
- Replaced extractor self-tests with `validate_loop_condition_structure` tests.

## Remaining Queue Impact

The extractor/detector residue queue is now narrower:

- `loop_simple_while.rs` keeps only the live shared helper
- `if_else_phi.rs` already keeps only its live detector surface

Remaining MIR structural vocabulary is now concentrated in active holds such as
`cond_profile`, `hints`, and any future owner-backed LocalSSA work.

## Proof

- `rg -n "extract_loop_simple_while_parts|LoopSimpleWhileParts|validate_loop_condition_structure" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
