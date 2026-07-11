# 291x-733 LoopBreak Minimal Lowerer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/common.rs`
- `src/mir/join_ir/lowering/common/README.md`
- `src/mir/join_ir/lowering/common/{balanced_depth_scan_emitter,condition_only_emitter,body_local_derived_emitter,body_local_derived_slot_emitter,conditional_step_emitter}.rs`
- `src/mir/join_ir/lowering/carrier_update_emitter/*`
- `src/mir/join_ir/lowering/loop_with_break_minimal.rs`
- `src/mir/join_ir/lowering/loop_with_break_minimal/*`
- `src/mir/join_ir/lowering/step_schedule.rs`
- `src/mir/policies/balanced_depth_scan/{classify,types}.rs`
- `docs/development/current/main/design/joinir-design-map.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The hand-written `loop_with_break_minimal` JoinIR lowerer had no live caller
outside its own unit test module. Current loop-break lowering is owned by the
plan/composer route path, so keeping this Phase 188 shelf module made an old
direct lowerer look like supported surface.

## Decision

Delete the unused lowerer module and its private submodules. Also delete the
dependent common emitters and schedule helper that had no caller once the
lowerer shelf was removed.

Move `BalancedDepthScanRecipe` to the policy result type module before deleting
`balanced_depth_scan_emitter.rs`, because non-test policy code still constructs
and stores that recipe vocabulary.

## Changes

- Removed `pub mod loop_with_break_minimal`.
- Deleted the unused lowerer entry, tests, and private implementation files.
- Removed `step_schedule` and the common emitters that were only reachable
  through the deleted lowerer.
- Removed the now-unused `carrier_update_emitter` module that was only exposed
  by the deleted lowerer shelf.
- Moved `BalancedDepthScanRecipe` from lowering/common into
  `mir::policies::balanced_depth_scan::types`, then deleted the unused balanced
  depth emitter.
- Removed stale current-design links to the deleted ConditionOnly/BodyLocal
  emitter files.
- Advanced `CURRENT_STATE.toml` to 291x-733.

## Proof

- `rg -n "lower_loop_with_break_minimal|LoopWithBreakLoweringInputs|loop_with_break_minimal" src tests tools -g '*.rs' -g '*.sh'`
- `rg -n "ConditionOnlyRecipe|ConditionOnlyEmitter|BodyLocalDerivedRecipe|BodyLocalDerivedEmitter|BodyLocalDerivedSlotRecipe|BodyLocalDerivedSlotEmitter|emit_conditional_step_update|LoopBreakScheduleFactsBox|step_schedule" src tests tools -g '*.rs' -g '*.sh'`
- `rg -n "balanced_depth_scan_emitter|condition_only_emitter|body_local_derived_emitter|body_local_derived_slot_emitter|conditional_step_emitter" src/mir docs/development/current/main/design -g '*.rs' -g '*.md'`
- `rg -n "carrier_update_emitter|emit_carrier_update_with_env" src tests tools -g '*.rs' -g '*.sh'`
- `rg -n "BalancedDepthScanRecipe" src/mir -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
