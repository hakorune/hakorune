# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-22
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長文履歴は archive に逃がす。
- cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cr/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`
- `tools/checks/dev_gate.sh runtime-exec-zero`

## Current Priority

- `phase-29cr` P5: packaging slice on the MIR substrate IDs / future crate boundaries
- landed slice:
  - `hakorune_mir_core` package: `types.rs` / `value_id.rs`
  - `hakorune_mir_core` package: `effect.rs`
  - `hakorune_mir_core` package: `basic_block_id.rs` / `binding_id.rs`
  - `hakorune_mir_core` package: `value_kind.rs`
  - `hakorune_mir_defs` package: `definitions/call_unified.rs`
  - `hakorune_mir_builder` package: `core_context.rs` / `context.rs`
  - `hakorune_mir_builder` package: `binding_context.rs`
  - `hakorune_mir_builder` package: `type_context.rs`
  - `hakorune_mir_builder` package: `variable_context.rs`
  - `hakorune_mir_builder` package: `metadata_context.rs`
  - builder / edgecfg / optimizer / tests now use public `crate::mir::{BasicBlockId, EdgeArgs}`
  - backend/mir_interpreter now uses public `crate::mir::BasicBlock` / `BasicBlockId`
  - `src/mir/contracts/README.md`
  - `src/mir/control_tree/README.md`
  - `src/mir/join_ir_vm_bridge/README.md`
  - `src/mir/join_ir_vm_bridge_dispatch/README.md`
  - `src/mir/policies/README.md`
    -> `hakorune-mir-*` future-name cleanup
- landed slice:
  - `box_arithmetic.rs` -> `pub mod box_arithmetic { ... }` inline facade
  - `box_operators.rs` -> `src/boxes/operators/`
  - `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`
  - `box_trait.rs` -> `src/boxes/box_trait.rs`
  - `operator_traits.rs` -> `src/boxes/operator_traits.rs`
  - `channel_box.rs` / `environment.rs` / `exception_box.rs` / `finalization.rs`
    / `instance_v2.rs` / `method_box.rs` / `scope_tracker.rs` / `type_box.rs`
    / `value.rs` / `ast.rs` / `benchmarks.rs` / `wasm_test.rs`
    -> directory modules
  - `src/mir/README.md`
  - `src/mir/builder/README.md`
  - `src/mir/join_ir/README.md`
  - `src/mir/loop_canonicalizer/README.md`
  - `src/mir/passes/README.md`
  - `src/mir/control_tree/README.md`
  - `src/mir/control_tree/step_tree/README.md`
  - `src/mir/control_tree/normalized_shadow/README.md`
  - `src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs`
  - `src/mir/builder/control_flow/plan/normalizer/helpers_layout.rs`
  - `src/mir/builder/control_flow/plan/normalizer/helpers_value.rs`
  - `src/mir/passes/rc_insertion.rs` -> facade
  - `src/mir/passes/rc_insertion_helpers.rs` -> implementation split
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_common.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_break_if.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_realworld.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_local.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_condition.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_helpers_loop.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_break_trim_whitespace_helpers.rs`
- next exact files:
  - `docs/development/current/main/design/mir-crate-split-prep-ssot.md`
- `src/mir/README.md`
- `src/mir/builder/README.md`
- `src/mir/passes/README.md`
- `src/mir/join_ir/README.md`
- `src/mir/contracts/README.md`
- `src/mir/policies/README.md`
- `src/mir/loop_canonicalizer/README.md`
- `src/mir/control_tree/README.md`
- `src/mir/join_ir_vm_bridge/README.md`
- `src/mir/join_ir_vm_bridge_dispatch/README.md`
- `crates/hakorune_mir_core/README.md`
- `crates/hakorune_mir_core/src/lib.rs`
- `crates/hakorune_mir_core/src/basic_block_id.rs`
- `crates/hakorune_mir_core/src/binding_id.rs`
- `crates/hakorune_mir_core/src/value_kind.rs`
- `src/mir/builder/type_context.rs`
- `crates/hakorune_mir_builder/src/variable_context.rs`
- `src/mir/builder/builder_value_kind.rs`
- `crates/hakorune_mir_defs/README.md`
- `crates/hakorune_mir_defs/src/lib.rs`
- `crates/hakorune_mir_builder/README.md`
- `crates/hakorune_mir_builder/src/lib.rs`
- `crates/hakorune_mir_builder/src/core_context.rs`
  - `crates/hakorune_mir_builder/src/context.rs`
  - `crates/hakorune_mir_builder/src/binding_context.rs`
  - `crates/hakorune_mir_builder/src/type_context.rs`
  - `src/mir/builder/variable_context.rs`
  - `src/mir/builder/scope_context.rs`
  - `src/mir/builder/compilation_context.rs`
- keep-root allowlist:
  - `basic_test.hako`
  - `test.hako`
- archive now:
  - `docs/archive/cleanup/root-hygiene/`
  - `tools/archive/root-hygiene/`
- P0 landed:
  - root archive relocation
  - `*.err` / `*.backup*` ignore policy

## Lane Pointers

- `phase-29cm`: collection owner cutover = done-enough stop line
- `phase-29y`: runtime `.hako` migration / boxcall contract = parked strict-polish
- `phase-21_5`: raw substrate perf = parked until boundary deepens
- `phase-29cr`: repo physical cleanup lane = active through P5 crate split prep
- `phase-29cr`: repo physical cleanup lane = active through P6 naming cleanup

## P5 / P6 Remaining Order

1. `scope_context.rs` (blocked until the `MirFunction` / lexical-scope seam is split further)
2. `compilation_context.rs`
3. `join_ir/` packaging boundary review
4. `passes/` packaging boundary review
5. remaining `hakorune-mir-*` naming surface polish

## Archive

- full snapshot archive:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
- prior archives:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
