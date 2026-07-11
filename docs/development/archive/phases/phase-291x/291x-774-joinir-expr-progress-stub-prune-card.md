# 291x-774 JoinIR Expr/Progress Stub Surface Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/verify.rs`
- `src/mir/join_ir/lowering/expr_lowerer/{mod.rs,types.rs,lowerer.rs}`
- `src/mir/join_ir/lowering/condition_lowering_box.rs`
- `src/mir/join_ir/lowering/loop_scope_shape/structural.rs`
- `CURRENT_STATE.toml`

## Why

The remaining local allowances included three inactive or test-only surfaces:

- `verify_progress_generic`, a Phase 30 placeholder with no callers.
- `ExprContext::General` and `ExprLowerer::builder`, neither used by the active
  Condition-only lowering contract.
- `LoopStructuralAnalysis` accessors used only by local tests.

## Decision

Remove unimplemented production API shelves instead of holding them under
`#[allow(dead_code)]`. Keep caller compatibility for `ExprLowerer::new` by
accepting the builder parameter but not storing it.

Structural-analysis accessors remain available under `#[cfg(test)]`.

## Landed

- Removed `verify_progress_generic`.
- Removed `ExprContext::General`.
- Removed the stored `builder` field from `ExprLowerer`.
- Updated ExprLowerer docs to describe Condition-only scope.
- Gated `LoopStructuralAnalysis::{exit_analysis,has_progress_carrier}` with
  `#[cfg(test)]`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes five local `#[allow(dead_code)]` items. Remaining holds are the
top-level JoinIR loop shape vocabulary and `LoopScopeShape::body`.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
