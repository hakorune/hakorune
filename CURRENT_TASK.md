# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-23
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長文履歴は archive に逃がす。
- naming cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cs/README.md`。
- substrate capability ladder lane の単一正本は `docs/development/current/main/phases/phase-29ct/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`
- `tools/checks/dev_gate.sh runtime-exec-zero`

## Current Priority

- `phase-29ct` substrate capability ladder docs/taskization is active
- `phase-29cm` collection owner cutover is done-enough stop line
- `phase-21_5` raw substrate perf reopen is parked until capability ladder + ABI/value manifest lock land
- `phase-29cs` kernel / plugin naming cleanup is parked
- future packaging/distribution pointer (docs-first, not active):
  - `docs/development/current/main/design/hakoruneup-release-distribution-ssot.md`
- current docs exact leaf:
  - `substrate-capability-ladder-ssot.md`
  - `value-repr-and-abi-manifest-ssot.md`
  - `abi-export-inventory.md`
  - `handle-cache-metal-helper-contract-ssot.md`
  - `minimal-capability-modules-ssot.md`
  - `minimum-verifier-ssot.md`
  - `raw-array-substrate-ssot.md`
  - `raw-map-substrate-ssot.md`
  - `gc-tls-atomic-capability-ssot.md`
  - `final-metal-split-ssot.md`
  - `phase-29ct/README.md`
- landed slice:
  - `phase-29ct` V0 ABI export inventory
    - docs-side truth now lives in
      `docs/development/current/main/design/abi-export-inventory.md`
    - current collection/kernel symbols are grouped as:
      `mainline substrate` / `runtime-facade` / `compat-only`
    - `AbiAdapterRegistryBox` is fixed as adapter-default consumer, not manifest truth
  - `phase-29ct` V1 value representation lock
    - canonical classes are fixed as:
      `imm_i64` / `imm_bool` / `handle_owned` /
      `handle_borrowed_string` / `boxed_local`
    - `value_public` stays V0 inventory umbrella only
    - `BorrowedHandleBox` is fixed as the current borrowed-string alias carrier
    - `CodecProfile` is fixed as helper policy, not public ABI schema
  - `phase-29ct` V2 metal helper contract lock
    - docs-side truth now lives in
      `docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md`
    - `handle_cache.rs` is fixed as:
      `typed handle cache` / `typed dispatch helper` / `array i64 re-encode helper`
    - non-goals:
      not ABI manifest truth / not value representation owner /
      not array-map policy owner
  - `phase-29ct` V3 future substrate module root lock
    - physical staging root is fixed at:
      `lang/src/runtime/substrate/README.md`
      `lang/src/runtime/substrate/hako_module.toml`
    - `runtime/substrate/` is namespace-first / docs-first only
    - no capability implementation has landed there yet
  - `phase-29ct` V4 minimal capability modules
    - docs-side truth now lives in
      `docs/development/current/main/design/minimal-capability-modules-ssot.md`
    - first staged order is fixed as:
      `mem -> buf -> ptr -> verifier`
    - physical staging docs exist under:
      `lang/src/runtime/substrate/mem/`
      `lang/src/runtime/substrate/buf/`
      `lang/src/runtime/substrate/ptr/`
  - `phase-29ct` V5 minimum verifier lock
    - docs-side truth now lives in
      `docs/development/current/main/design/minimum-verifier-ssot.md`
    - verifier order is fixed as:
      `bounds -> initialized-range -> ownership`
    - physical staging root now exists at:
      `lang/src/runtime/substrate/verifier/README.md`
  - `phase-29ct` C2 RawArray docs/task lock
    - docs-side truth now lives in
      `docs/development/current/main/design/raw-array-substrate-ssot.md`
    - `RawArray` is fixed as the first algorithm-substrate consumer of:
      `hako.mem` / `hako.buf` / `hako.ptr` / minimum verifier
    - physical staging root now exists at:
      `lang/src/runtime/substrate/raw_array/README.md`
  - `phase-29ct` C3 RawMap docs/task lock
    - docs-side truth now lives in
      `docs/development/current/main/design/raw-map-substrate-ssot.md`
    - `RawMap` is fixed as the next algorithm-substrate consumer after:
      `RawArray`
    - physical staging root now exists at:
      `lang/src/runtime/substrate/raw_map/README.md`
  - `phase-29ct` C4 GC/TLS/atomic capability widening lock
    - docs-side truth now lives in
      `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
    - current widening order is fixed as:
      `atomic -> tls -> gc`
    - physical staging roots now exist at:
      `lang/src/runtime/substrate/atomic/README.md`
      `lang/src/runtime/substrate/tls/README.md`
      `lang/src/runtime/substrate/gc/README.md`
  - `phase-29ct` C6 final metal split detail lock
    - docs-side truth now lives in
      `docs/development/current/main/design/final-metal-split-ssot.md`
    - final boundary is now read through:
      `.hako owner` / `native metal keep` / `not yet moved`
    - `C5 Hakozuna portability layer` remains ladder-only and deferred
  - `phase-29ct` I1 capability stubs + RawArray probe path
    - `lang/src/runtime/substrate/mem/mem_core_box.hako`
      now exposes the first live `hako.mem` bridge
    - `lang/src/runtime/substrate/buf/buf_core_box.hako`
      now exposes the first live `hako.buf` bridge
    - `lang/src/runtime/substrate/ptr/ptr_core_box.hako` now owns the first live
      `slot_load_i64` / `slot_store_i64` capability hop
    - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako` now owns the first
      runnable algorithm-substrate probe path
    - `ArrayCoreBox.get_i64/set_i64` now route through `RawArrayCoreBox`
  - `phase-29ct` I2 RawArray len/append widening
    - `lang/src/runtime/substrate/ptr/ptr_core_box.hako` now also owns
      `slot_len_i64` / `slot_append_any`
    - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako` now widens the
      runnable substrate path to `get/set/len/push`
    - `ArrayCoreBox.len_i64/push_hh` now route through `RawArrayCoreBox`
  - `phase-29ct` I3 RawArray reserve/grow slice
    - `lang/src/runtime/substrate/ptr/ptr_core_box.hako` now also owns
      `slot_reserve_i64` / `slot_grow_i64`
    - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako` now widens the
      substrate capacity vocabulary through `BufCoreBox`
    - `RawArray` is now the first consumer of `reserve/grow` as well as slot access
  - `phase-29ct` I4 live mem/buf facade slice
    - `lang/src/runtime/substrate/mem/mem_core_box.hako` now exposes the live
      `alloc_i64` / `realloc_i64` / `free_i64` bridge
    - `lang/src/runtime/substrate/buf/buf_core_box.hako` now exposes the live
      `len_i64` / `cap_i64` / `reserve_i64` / `grow_i64` surface
    - `RawArrayCoreBox.slot_reserve_i64/slot_grow_i64` now route through `BufCoreBox`
  - `phase-29ct` I5 minimum verifier bounds slice
    - docs-side truth now lives in
      `docs/development/current/main/design/minimum-verifier-ssot.md`
    - first live verifier box is
      `lang/src/runtime/substrate/verifier/bounds/bounds_core_box.hako`
    - `RawArrayCoreBox.slot_load_i64/slot_store_i64` now gate through
      `BoundsCoreBox` before `PtrCoreBox`
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
  - `docs/development/current/main/phases/phase-29ct/README.md`
  - `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
  - `docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md`
  - `docs/development/current/main/design/abi-export-inventory.md`
  - `docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md`
  - `docs/development/current/main/design/minimal-capability-modules-ssot.md`
  - `docs/development/current/main/design/minimum-verifier-ssot.md`
  - `docs/development/current/main/design/raw-array-substrate-ssot.md`
  - `docs/development/current/main/design/raw-map-substrate-ssot.md`
  - `docs/development/current/main/design/gc-tls-atomic-capability-ssot.md`
  - `docs/development/current/main/design/final-metal-split-ssot.md`
  - `lang/src/runtime/substrate/mem/mem_core_box.hako`
  - `lang/src/runtime/substrate/buf/buf_core_box.hako`
  - `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
  - `lang/src/runtime/substrate/verifier/bounds/bounds_core_box.hako`
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `lang/src/runtime/collections/array_core_box.hako`
  - `docs/development/current/main/phases/phase-29cm/README.md`
  - `docs/development/current/main/design/collection-raw-substrate-contract-ssot.md`
  - `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
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
- `phase-21_5`: raw substrate perf = parked behind capability ladder docs/task lock
- `phase-29cr`: repo physical cleanup lane = stop-line reached
- `phase-29cs`: kernel / plugin naming cleanup = parked
- `phase-29ct`: substrate capability ladder = active docs/task lane

## P5 / P6 / Perf / Capability Stop-Line

1. `scope_context.rs`: parked until the `MirFunction` / lexical-scope seam is split explicitly
2. `compilation_context.rs`: parked (mixed ownership / ASTNode + FunctionSlotRegistry + TypeRegistry)
3. `join_ir/` whole-subtree packaging: parked docs-first only
4. `passes/` whole-subtree packaging: parked docs-first only
5. remaining `hakorune-mir-*` naming polish: optional, low-value
6. cleanup lane can park after the naming cleanup phase lands
7. capability ladder docs/task lock comes before deeper substrate or allocator migration
8. perf reopen stays parked until capability ladder + ABI/value manifest lock land

## Archive

- full snapshot archive:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
- prior archives:
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
