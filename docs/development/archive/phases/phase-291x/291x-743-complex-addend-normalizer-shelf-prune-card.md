# 291x-743 Complex Addend Normalizer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/complex_addend_normalizer.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`complex_addend_normalizer` was an old AST preprocessing surface for rewriting
carrier updates before analysis. It had no live Rust callers outside its own
tests. Keeping it exposed a dead AST-rewrite vocabulary in a lane that now keeps
loop observation conservative and analysis-only.

## Decision

Delete the module and remove the lowering module export. Active loop update
handling remains owned by the current analysis/lowering path.

## Changes

- Removed `complex_addend_normalizer.rs`.
- Removed `pub mod complex_addend_normalizer`.
- Advanced `CURRENT_STATE.toml` to 291x-743.

## Proof

- `rg -n "complex_addend_normalizer|ComplexAddendNormalizer|NormalizationResult" src tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
