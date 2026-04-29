# 291x-735 IfPhi If-Sum Lowerer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/loop_with_if_phi_if_sum/*`
- `src/mir/join_ir/lowering/condition_pattern.rs`
- `docs/development/current/main/design/compiler-task-map-ssot.md`
- `docs/development/current/main/phases/phase-251/README.md`
- `docs/development/current/main/phases/phase-264/README.md`
- `docs/development/current/main/phases/phase-291x/291x-725-if-phi-loopform-stub-prune-card.md`
- `docs/development/current/main/phases/phase-291x/291x-727-loopform-route-router-shelf-prune-card.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The `loop_with_if_phi_if_sum` direct JoinIR lowerer had no live Rust caller
outside its own unit tests. Current IfPhiJoin lowering is owned by the
`joinir::route_entry` registry and `RecipeComposer::compose_if_phi_join_recipe`,
so this old AST direct lowerer kept retired surface visible as active code.

## Decision

Delete the IfPhi if-sum direct lowerer shelf and remove its module export. Update
current docs and nearby comments to point at the recipe/facts route instead.

## Changes

- Removed the `loop_with_if_phi_if_sum` module export.
- Deleted the lowerer module, extractor helper, and self-contained tests.
- Updated IfPhiJoin current owner docs to reference route-entry registry,
  `RecipeComposer`, and IfPhiJoin Facts.
- Updated stale phase-291x cards that described the old module as live.
- Advanced `CURRENT_STATE.toml` to 291x-735.

## Proof

- `rg -n "lower_if_sum_pattern|IfSumConditionBinding" src tests tools -g '*.rs' -g '*.sh'`
- `rg -n "pub mod loop_with_if_phi_if_sum" src/mir/join_ir/lowering/mod.rs`
- `rg -n "Live IfPhiJoin lowering remains.*loop_with_if_phi_if_sum|Current route-family surfaces:.*loop_with_if_phi_if_sum|current route.*loop_with_if_phi_if_sum" docs/development/current/main -g '*.md' -g '!**/291x-735-if-phi-if-sum-lowerer-shelf-prune-card.md'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
