# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-22
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長文履歴は archive に逃がす。
- cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cs/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`
- `tools/checks/dev_gate.sh runtime-exec-zero`

## Current Priority

- `phase-29cs` kernel / plugin naming cleanup is active
- next mainline return after this lane: `.hako` kernel migration (`phase-29cm`)
- landed slice:
  - compat/pure append retarget: `AbiAdapterRegistryBox` default `ArrayBox.push`
    and historical pure `ArrayBox.push -> len` lowering now use
    `nyash.array.slot_append_hh`; `nyash.array.push_h` remains compat-only
  - compat/pure map retarget: `AbiAdapterRegistryBox` default `MapBox.get/set/has`
    and historical pure `MapBox.{get,set,has}` lowering now use
    `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh` / `nyash.map.probe_hh`;
    `nyash.map.{get_h,set_h,has_h}` remain compat-only
  - compat/pure array get retarget: `AbiAdapterRegistryBox` default `ArrayBox.get`
    and historical pure `ArrayBox.get` lowering now use `nyash.array.slot_load_hi`;
    `nyash.array.get_h` remains compat-only
  - compat/pure array set retarget: `AbiAdapterRegistryBox` default `ArrayBox.set`
    and historical pure `ArrayBox.set` lowering now use `nyash.array.set_hih`;
    `nyash.array.set_h` remains compat-only
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
  - `hakorune_mir_joinir` package: `join_ir/ownership/types.rs`
  - `src/mir/join_ir/ownership/bridge/` now groups `plan_to_lowering.rs` /
    `plan_validator.rs` behind the ownership facade
  - `src/mir/join_ir/ownership/analyzer/` now groups ProgramJSON analysis into
    `mod.rs` / `core.rs` / `node_analysis.rs`
  - `src/mir/passes/concat3_canonicalize/analysis/` now groups
    `stringish.rs` / `def_use.rs` behind the pass facade
  - `scope_context.rs` is still parked as-is:
    `current_function` / `function_param_names` still tie it to the
    function-lifecycle seam
  - `compilation_context.rs` is parked: mixed ownership (`ASTNode` / `FunctionSlotRegistry` / `TypeRegistry`)
  - builder / edgecfg / optimizer / tests now use public `crate::mir::{BasicBlockId, EdgeArgs}`
  - backend/mir_interpreter now uses public `crate::mir::BasicBlock` / `BasicBlockId`
  - `src/mir/contracts/README.md`
  - `src/mir/control_tree/README.md`
  - `src/mir/join_ir_vm_bridge/README.md`
  - `src/mir/join_ir_vm_bridge_dispatch/README.md`
  - `src/mir/policies/README.md`
    -> `hakorune-mir-*` future-name cleanup
  - `src/mir/join_ir/` is docs-first only for now:
    AST/ProgramJSON + runtime/env + MIR coupling still blocks a safe package move,
    `json.rs` keeps JoinIR serialization in the same review lane, and the
    `join_ir_vm_bridge/` boundary is not yet stable enough for split
    sub-box order is:
    `ownership_types -> ownership analyzer core -> ownership bridge glue
    -> lowering substrate -> condition cluster -> loop-route cluster
    -> target-specific lowerers -> bridge fence`
  - `src/mir/passes/` is docs-first only for now:
    AST/runtime/config/env + MIR coupling still blocks a safe package move,
    `rc_insertion/` is especially blocked, and `concat3_canonicalize/` is only
    a future extraction candidate
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
  - `docs/development/current/main/phases/phase-29cs/README.md`
  - `docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md`
  - `crates/nyash_kernel/src/plugin/mod.rs`
  - `crates/nyash_kernel/src/plugin/array_index_dispatch.rs`
  - `crates/nyash_kernel/src/plugin/array_write_dispatch.rs`
  - `crates/nyash_kernel/src/plugin/handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data_map_dispatch.rs`
  - `src/bin/hakorune_compat.rs`
  - `Cargo.toml`
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
- `phase-29cr`: repo physical cleanup lane = stop-line reached
- `phase-29cs`: kernel / plugin naming cleanup = active

## P5 / P6 Stop-Line

1. `scope_context.rs`: parked until the `MirFunction` / lexical-scope seam is split explicitly
2. `compilation_context.rs`: parked (mixed ownership / ASTNode + FunctionSlotRegistry + TypeRegistry)
3. `join_ir/` whole-subtree packaging: parked docs-first only
4. `passes/` whole-subtree packaging: parked docs-first only
5. remaining `hakorune-mir-*` naming polish: optional, low-value
6. cleanup lane can park after the naming cleanup phase lands

## Archive

- full snapshot archive:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
- prior archives:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
