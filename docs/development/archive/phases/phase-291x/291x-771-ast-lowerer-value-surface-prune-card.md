# 291x-771 AST Lowerer Value Surface Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/frontend/ast_lowerer/mod.rs`
- `src/mir/join_ir/frontend/ast_lowerer/expr.rs`
- `CURRENT_STATE.toml`

## Why

`AstToJoinIrLowerer::extract_value` still carried a stale
`#[allow(dead_code)]`, but current loop/if routes call it directly.

`AstToJoinIrLowerer::next_var_id` was a separate unused field; active value ID
allocation is owned by `ExtractCtx::next_var_id`.

## Decision

Remove the stale allowance and delete the dead lowerer-level counter field.
Keep value allocation ownership in `ExtractCtx`.

## Landed

- Removed `AstToJoinIrLowerer::next_var_id`.
- Removed the stale dead-code allowance from `extract_value`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes two AST lowerer value-surface stale items without changing route
behavior.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
