# 291x-745 DigitPos Condition Normalizer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/digitpos_condition_normalizer.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`digitpos_condition_normalizer` was an old AST rewrite helper that transformed
`digit_pos < 0` into `!is_digit_pos`. It had no live Rust callers outside its
own tests. Keeping it as a public lowering module preserved a stale rewrite
surface after the active DigitPos handling moved into the current detector and
analysis path.

## Decision

Delete the module and remove its lowering export. The compiler cleanup lane
keeps condition observation analysis-only and avoids dormant AST mutation hooks.

## Changes

- Removed `digitpos_condition_normalizer.rs`.
- Removed `pub mod digitpos_condition_normalizer`.
- Advanced `CURRENT_STATE.toml` to 291x-745.

## Proof

- `rg -n "digitpos_condition_normalizer|DigitPosConditionNormalizer" src tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
