# 291x-754 Old CoreLoop Composer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- old cfg-test `composer/coreloop_v0*`
- old cfg-test `composer/coreloop_v1*`
- old cfg-test `composer/coreloop_single_entry`
- old cfg-test value-join gate helpers in `composer/coreloop_gates.rs`
- old cfg-test `normalizer/simple_while_coreloop_builder`
- active docs that still described the old single-entry composer as current
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The worker inventory identified the old coreloop v0/v1 composer shelf as a
test-surface/reconcile candidate. It was already cfg-test only and was not part
of the release planner/composer path. It also kept stale facts vocabulary alive
inside tests (`condition_shape`, cleanup Return rejection slots, old v0/v1 gate
semantics).

The active composer path is recipe-first:

- route-specific Facts/Recipe/Composer owners
- `recipe_tree/*_composer.rs`
- `composer/coreloop_v2_nested_minimal.rs` for the still-live nested minimal composer

## Decision

Retire the old cfg-test coreloop v0/v1/single-entry shelf rather than blessing
it as historical regression coverage.

Do not touch `coreloop_v2_nested_minimal` or `coreloop_gates`.

## Landed

- Removed `coreloop_single_entry.rs`.
- Removed `coreloop_v0.rs` and `coreloop_v0_tests/`.
- Removed `coreloop_v1.rs` and `coreloop_v1_tests/`.
- Removed `normalizer/simple_while_coreloop_builder.rs`.
- Removed cfg-test-only v0/v1 gate helpers from `coreloop_gates.rs`.
- Removed their cfg-test module declarations.
- Marked the old CoreLoop single-entry SSOT as retired.

## Remaining Queue Impact

The old coreloop v0/v1 test-surface item is closed. Remaining structural
cleanup is now narrower:

- `LoopFacts::condition_shape`
- `CleanupKindFacts::Return`
- `SkeletonKind::{If2,BranchN}`
- bridge strict/env/LowerOnly semantics
- `condition_lowering_box` / `condition_to_joinir` / `update_env`

## Proof

- `rg -n "coreloop_single_entry|coreloop_v0|coreloop_v1|simple_while_coreloop_builder|build_simple_while_coreloop" src/mir/builder/control_flow/plan -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
