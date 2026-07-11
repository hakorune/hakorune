# 291x-751 Plan Test Helper Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/plan/condition_env_builder.rs`
- `src/mir/builder/control_flow/plan/trim_lowerer.rs`
- `src/mir/builder/control_flow/plan/trim_validator.rs`
- `src/mir/builder/control_flow/plan/mod.rs`
- active plan layout docs and route-entry comments
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

These helpers had already been narrowed to `#[cfg(test)]` in 291x-703 and
291x-705. A fresh inventory showed that the only remaining code references were
their own self-tests. Keeping module declarations for them made old route-helper
surfaces appear active even though current route ownership lives in
Facts/Recipe/Composer and route-specific recognizers.

## Decision

Delete the modules instead of keeping test-only shelves.

Do not touch active trim/condition route support outside these three files.

## Landed

- Removed `condition_env_builder.rs`.
- Removed `trim_lowerer.rs`.
- Removed `trim_validator.rs`.
- Removed their module declarations from `plan/mod.rs`.
- Updated active plan layout docs and route-entry comments to mark these helper
  shelves as retired.

## Remaining Queue Impact

This closes the cfg-test trim/condition helper quarantine item from the worker
inventory. Remaining planner work is structural:

- `LoopFacts::condition_shape`
- `SplitScanFacts::shape`
- `CleanupKindFacts::Return`
- `SkeletonKind::{If2,BranchN}`
- old coreloop v0/v1 composer tests

## Proof

- `rg -n "ConditionEnvBuilder|TrimLowerer|TrimValidator|condition_env_builder|trim_lowerer|trim_validator" src tests docs/development/current/main -g '*.rs' -g '*.md'`
  - current hits are retired-note docs, phase history/archive, or this card only
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
