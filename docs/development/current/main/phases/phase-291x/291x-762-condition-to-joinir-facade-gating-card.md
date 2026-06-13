# 291x-762 ConditionToJoinIR Facade Gating Card

Status: Superseded
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup
Superseded-By: JOINIR-THIN-002 in
`docs/development/current/main/design/compiler-pipeline-thinning-ssot.md`

## Scope

- `condition_to_joinir.rs`
- inline-boundary imports
- JoinIR README
- `CURRENT_STATE.toml`

## Why

The `condition_to_joinir` module had become a re-export facade. Active condition
lowering calls use `condition_lowerer` directly, while inline-boundary code only
needed `ConditionBinding` through the facade.

Keeping the facade always built made ownership ambiguous between
`condition_lowerer`, `condition_env`, and the facade module.

## Decision

Make `condition_lowerer` and `condition_env` the production owners:

- condition lowering API: `condition_lowerer`
- condition binding/env types: `condition_env`
- legacy API facade: `condition_to_joinir.rs` under `#[cfg(test)]`

Follow-up decision:

- `JOINIR-THIN-002` retired `condition_to_joinir.rs` entirely after usage
  inventory showed no external references.
- Tests now exercise `lowering/condition_lowerer/*` directly.

## Landed

- Gated `lowering::condition_to_joinir` with `#[cfg(test)]`.
- Repointed inline-boundary imports to `condition_env::ConditionBinding`.
- Updated the JoinIR README to mark the facade as test-only legacy.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The `condition_to_joinir` production-facade item is closed and the test-only
facade is retired. Remaining structural cleanup is now:

- bridge strict/env/LowerOnly semantics
- broad JoinIR lowering module-level `dead_code` allowance inventory

## Proof

- `rg -n "condition_to_joinir::" src/mir src/tests -g '*.rs'`
- `rg -n "#\\[cfg\\(test\\)\\]|pub mod condition_to_joinir" src/mir/join_ir/lowering/mod.rs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
