# 291x-732 ConditionOnly Trim-Helper Compat Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/common/condition_only_emitter.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`ConditionOnlyRecipe::from_trim_helper()` was a deprecated compatibility wrapper
that always selected the `WhenMatch` constructor. Current call sites use the
explicit `from_trim_helper_condition_only()` and
`from_trim_helper_normal_trim()` constructors, so the wrapper kept an ambiguous
entry point alive without an owner path.

## Decision

Delete the wrapper and leave the two explicit constructors as the only public
ConditionOnly recipe entries.

## Changes

- Removed `ConditionOnlyRecipe::from_trim_helper()`.
- Removed its local `#[allow(dead_code)]`.
- Advanced `CURRENT_STATE.toml` to 291x-732.

## Proof

- `rg -n "from_trim_helper\\(" src/mir/join_ir/lowering/common/condition_only_emitter.rs src/mir/join_ir/lowering/loop_with_break_minimal src tests tools -g '*.rs' -g '*.sh'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
