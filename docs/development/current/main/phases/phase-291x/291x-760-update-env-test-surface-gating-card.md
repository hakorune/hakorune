# 291x-760 UpdateEnv Test Surface Gating Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/update_env.rs`
- JoinIR lowering README files
- body-local environment comments
- `CURRENT_STATE.toml`

## Why

`UpdateEnv` was listed as a remaining JoinIR lowering reconcile candidate. The
current codebase has no production imports of `UpdateEnv`; the remaining uses
are its own unit tests and historical docs/comments.

Keeping it as an always-built public module made it look like an active name
resolution owner next to `ScopeManager`.

## Decision

Do not delete the file in this card. Keep the old resolution behavior as a
test-only harness and make the production ownership explicit:

- active name resolution: `ScopeManager`
- condition value map: `ConditionEnv`
- body-local value map: `LoopBodyLocalEnv`
- legacy update resolution harness: `update_env.rs` under `#[cfg(test)]`

## Landed

- Gated `lowering::update_env` with `#[cfg(test)]`.
- Updated JoinIR lowering docs to mark `UpdateEnv` as test-only legacy harness.
- Updated body-local comments to point variable-priority ownership at
  `ScopeManager`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The `update_env` production-surface item is closed. Remaining structural cleanup
is now:

- `condition_lowering_box` / `condition_to_joinir`
- bridge strict/env/LowerOnly semantics

## Proof

- `rg -n "UpdateEnv::|use .*update_env" src/mir src/tests -g '*.rs' -g '!src/mir/join_ir/lowering/update_env.rs'`
- `rg -n "#\\[cfg\\(test\\)\\]|pub mod update_env" src/mir/join_ir/lowering/mod.rs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
