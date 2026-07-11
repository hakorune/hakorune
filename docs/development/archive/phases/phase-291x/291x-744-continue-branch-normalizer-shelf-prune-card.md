# 291x-744 Continue Branch Normalizer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/continue_branch_normalizer.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`continue_branch_normalizer` was an old AST rewrite helper that transformed
`if { body } else { continue }` into the inverse branch shape before lowering.
It had no live Rust callers outside its own tests, and keeping it exported made
the active route-shape vocabulary look broader than it is.

## Decision

Delete the module and remove its lowering export. Continue-only route handling
stays owned by the current detector/lowering pipeline without AST mutation.

## Changes

- Removed `continue_branch_normalizer.rs`.
- Removed `pub mod continue_branch_normalizer`.
- Advanced `CURRENT_STATE.toml` to 291x-744.

## Proof

- `rg -n "continue_branch_normalizer|ContinueBranchNormalizer" src tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
