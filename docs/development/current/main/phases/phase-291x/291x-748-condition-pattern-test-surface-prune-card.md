# 291x-748 Condition Pattern Test Surface Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/condition_pattern.rs`
- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/README.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

291x-747 worker inventory found that `condition_pattern` no longer has a
production caller. It remained as a public lowering module with self-tests and
historical condition-vocabulary prose, while current condition lowering goes
through `condition_lowerer` / `ExprLowerer` and route facts.

Keeping it as a public module made the old AST-based condition vocabulary look
like active production policy.

## Decision

Delete the stale module instead of moving it to `#[cfg(test)]`.

Reason:

- all code hits were the module declaration or self-tests
- current route vocabulary should live with active route facts and condition
  lowering, not an unused AST-pattern helper
- retaining it as test-only would keep a second vocabulary source alive

## Landed

- Removed `condition_pattern.rs`.
- Removed `pub mod condition_pattern`.
- Removed the module from the JoinIR README condition cluster.

## Remaining Queue Impact

`condition_pattern` is no longer in the remaining cleanup queue. The nearby
structural candidates remain separate:

- `condition_lowering_box`
- `condition_to_joinir`
- `update_env`
- planner facts condition/skeleton vocabulary

## Proof

- `rg -n "condition_pattern|ConditionPattern|ConditionCapability|analyze_condition_pattern|analyze_condition_capability|normalize_comparison|is_simple_comparison|NormalizedCondition|ConditionValue" src tests docs/development/current/main -g '*.rs' -g '*.md'`
  - current hits are phase history/archive or this card only
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
