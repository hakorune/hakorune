# 291x-775 JoinIR Shape Allowance Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/mod.rs`
- `src/mir/join_ir/lowering/loop_scope_shape/shape.rs`
- `CURRENT_STATE.toml`

## Why

After the preceding test-surface and stub cleanup, the only remaining local
dead-code allowances were on active JoinIR shape vocabulary:

- `LoopHeaderShape`
- `LoopHeaderShape` impl
- `LoopExitShape`
- `LoopExitShape` impl
- `LoopScopeShape::body`

Inventory showed these are now live enough through lowering code or local tests,
so the allowances no longer document a real hold.

## Decision

Remove the allowances without changing the shape vocabulary. Future dead-code
warnings in these shapes should be handled locally, not hidden by stale phase
comments.

## Landed

- Removed stale allowances from `LoopHeaderShape` and its impl.
- Removed stale allowances from `LoopExitShape` and its impl.
- Removed the stale allowance from `LoopScopeShape::body`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This clears the active JoinIR/compiler-cleanliness `#[allow(dead_code)]`
inventory scanned for this lane.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
