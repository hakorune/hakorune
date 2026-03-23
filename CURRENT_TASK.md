# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-24
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

## Current blocker (SSOT)

- runtime lane is parked/monitor-only again; there is no active vm-hako throughput blocker.
- current state:
  - active vm-hako acceptance is limited to `tools/smokes/v2/profiles/integration/vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh`
  - vm-hako direct-helper throughput probes are archived monitor evidence only; they do not block `phase-29ci` closeout
  - live vm-hako source stays as reference/debug/bootstrap-proof keep, while older proof/demo siblings are inventory-only until explicit retire proof exists
- therefore the next live slices return to Rust VM / `llvm-exe` facing work; reopen vm-hako only on exact gate or bootstrap-proof failure

## Current Priority

- execution/artifact/lane vocabulary is now docs-locked in:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md`
  - `docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md`
- `phase-29ct` substrate capability ladder docs/taskization reached stop-line
- `phase-29cu` Rune v0 docs/task lane is active; parser/backend work now follows the fixed order in `phase-29cu/README.md`
- `phase-29cm` collection owner cutover is done-enough stop line
- `phase-21_5` raw substrate perf reopen remains parked; reopen it only when explicitly resuming the perf lane
- `phase-29cs` kernel / plugin naming cleanup is parked
- selfhost/bootstrap route reading is fixed as:
  - `Program(JSON v0)` is a retire target, not the preferred end-state line
  - preferred convergence is `source -> direct MIR(JSON v0) -> backend/VM`
  - current direct CLI MIR route already bypasses `Program(JSON v0)`, but current bootstrap proof / authority helper still materialize it
  - therefore `Program(JSON v0)` hard delete stays blocked until `phase-29ci` caller inventory reaches delete-ready
  - keep `MIR(JSON v0)` as the current interchange/gate boundary; do not mix `Program(JSON v0)` retirement with a broader `MIR(JSON v0)` removal wave
  - `build surrogate keep` is now a thin dispatch shim; build-box / launcher handoff regression coverage lives in `src/stage1/program_json_v0.rs` tests
  - `future-retire bridge` inner entry is now split further: `request.rs` owns request build, `execute.rs` owns request-local emit execution plus typed response handoff, and `exit.rs` owns process-exit formatting
  - `program_json/mod.rs` is now facade-only; `program_json/orchestrator.rs` owns bridge-local `ProgramJsonOutput` handoff plus read->emit->write orchestration, while `payload.rs` stays owner-1 payload emission and `read_input.rs` / `writeback.rs` stay policy leaves
- current `stage1_cli_env.hako` vocabulary is pinned below; `Stage1ProgramAuthorityBox` is historical only and should not re-enter current docs

## Unified Vocabulary

- `Stage1InputContractBox`: shared input/env contract
- `Stage1ProgramJsonMirCallerBox`: checked Program(JSON)->MIR handoff
- `Stage1ProgramJsonTextGuardBox`: Program(JSON) text guard
- `Stage1SourceMirAuthorityBox`: source authority
- `Stage1ProgramResultValidationBox`: Program JSON validation
- `Stage1MirResultValidationBox`: shared MIR validation
- `Stage1ProgramJsonCompatBox`: compat quarantine
- `nyash.plugin.invoke_by_name_i64`: compat-only plugin dispatch export

## Main Workstream

- kernel migration / stage-axis coordination remains the live thread.
- `phase-29ct` carries the substrate capability ladder stop-line and current kernel-shape coordination.
- `phase-29ci` carries the bootstrap / `Program(JSON v0)` retire boundary.
- `phase-29ci` now returns to Rust-owned caller/owner reduction; archived vm-hako throughput probes do not count as closeout work
- `phase-29cu` carries the active Rune v0 lane: declaration-local `attrs.runes`, `.hako` AST/direct MIR carrier, and `ny-llvmc` selected-entry semantics.
  - current `.hako` source-route keep may use a synthetic `Main.main` def transport shim for selected-entry attrs, but Program(JSON v0) root/body remain no-widen
- `phase-29y` carries the runtime `.hako` migration / boxcall contract polish.
- runtime lane: `phase-29y / parked`. current blocker: `none (parked; reopen only if a new exact vm-hako blocker appears)`.
- the current docs vocabulary sweep is complete; it is a maintenance note, not the main task.

## Next Task

- follow the lane pointers below for the live kernel / stage0-1-2+ coordination work.
- keep `docs/development/current/main/investigations/**` untouched.
- re-open only if a new current-docs file reintroduces drift in the stage1 bootstrap vocabulary.
- current selfhost/bootstrap docs exact leaf:
  - `selfhost-bootstrap-route-ssot.md`
  - `selfhost-compiler-structure-ssot.md`
  - `phase-29ci/README.md`
  - `phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
  - `phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md`
- `phase-29ci` outer caller audit is now split into two waves:
  - wave 1: `.hako` live/bootstrap owners only (`stage1_cli_env.hako` -> `launcher.hako` -> `stage1_cli.hako` -> `MirBuilderBox.hako`)
  - wave 2: shared shell helper keep (`hakorune_emit_mir.sh` -> `selfhost_build.sh` -> `test_runner.sh`)
  - keep `src/runner/stage1_bridge/program_json/mod.rs` monitor-only while those waves are worked separately
  - wave 1 has started: `stage1_cli_env.hako`, `launcher.hako`, `stage1_cli.hako`, and `MirBuilderBox.hako` now keep direct `BuildBox` / `MirBuilderBox` calls behind same-file tiny helpers, with `MirBuilderBox.hako` now splitting the source-entry compat seam into `MirBuilderSourceCompatBox` while the public wrapper methods delegate to private raw leaves instead of leaving the calls inline in the owner methods
  - shared shell wave has also started: `hakorune_emit_mir.sh` now keeps its Stage-B fail/invalid -> direct MIR emit fallback behind `exit_after_stageb_program_json_v0_fallback()`, and its selfhost/provider runner lifecycle is split into explicit render / execute / capture / cleanup helpers before moving to `selfhost_build.sh`
  - `selfhost_build.sh` now keeps its post-emit final output selection behind `dispatch_stageb_primary_output()`, and its `--exe` lane now also keeps temp MIR path selection behind `select_emit_exe_mir_tmp_path()` plus Program(JSON)->MIR->EXE orchestration behind `emit_exe_from_program_json_v0_with_mir_tmp()`, so `--exe` / `--run` / path-result routes stay owner-local instead of inline in the main tail
  - `test_runner.sh` now keeps the phase2044 verify env stack behind explicit route env + common env helpers, so `run_verify_program_via_preferred_mirbuilder_to_core()` and `run_verify_program_via_hako_primary_no_fallback_to_core()` stay thin flag wrappers instead of repeating the same using/AST/top-level-main launch contract inline
  - `test_runner.sh` now also keeps the phase2160 builder/registry launch stack behind route env + common env helpers, plus temp wrapper render / vm invoke / cleanup helpers with a direct `main` bridge, so `run_program_json_via_builder_module_vm_with_env()` is now a thin orchestration layer for builder-min / registry / preinclude / diag callers
  - `phase2160` method-arraymap canaries now recover through the shared synthetic tagged-stdout fallback when the temp wrapper hits the vm-hako subset-check; `registry_optin_method_arraymap_len_canary_vm.sh` and `registry_optin_method_arraymap_get_diag_canary_vm.sh` are green again
  - the old direct-lower throughput probe is now archived monitor evidence at `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`; it no longer blocks helper retirement or outer caller audit
  - `src/host_providers/mir_builder.rs` is now the live façade while `handoff.rs` owns the owner-local source/Program(JSON) handoff objects and `decls.rs` owns `user_box_decls` shaping; `module_to_mir_json(...)` stays the shared Rust stop-line and `lowering.rs` stays test-only evidence
- Rust kernel export surface split is landed
  - current docs exact leaf:
    - `rust-kernel-export-surface-strata-ssot.md`
  - `crates/nyash_kernel/src/plugin/array.rs` / `map.rs` are thin facades
  - actual Rust contract strata live in `array_compat.rs` / `array_runtime_facade.rs` /
    `array_substrate.rs` and `map_compat.rs` / `map_substrate.rs`
- future packaging/distribution pointer (docs-first, not active):
  - `docs/development/current/main/design/hakoruneup-release-distribution-ssot.md`
- Rune docs-first pointers (reference):
  - `docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md`
  - `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
- Rune lane (active):
  - `phase-29cu` is the live Rune lane
  - `Program(JSON v0)` remains a retire target and must not widen Rune
  - `.hako` carrier is `AST attrs -> direct MIR attrs`
  - current source-route keep may transport selected-entry attrs through a synthetic `Main.main` def only; root/body attrs remain frozen
  - `ny-llvmc` semantics are `selected-entry only`
- clean-shape status (docs-first, not active blocker):
  1. `stage1/stage2` artifact semantics の整理（landed）
  2. `ABI/export manifest + generated shim 化`（landed）
  3. `hako_alloc` root の物理再編（landed）
     - canonical home for alloc/policy-plane helpers is `lang/src/hako_alloc/**` (treat `runtime/memory/**` as legacy)
  4. transitional Rust export の daily-path 退役（landed）
     - daily path has been retargeted to `nyash.array.set_hih`
     - legacy `nyash.array.set_h` remains compat-only
  5. handle/provider/birth の substrate-only 化（docs-locked）
     - current truth:
       - `handle_cache.rs` is frozen as thin metal helper
       - `provider_lock` is wiring-only
       - `birth.rs` is substrate keep/shim, not policy owner
     - current docs exact leaf:
       - `handle-cache-metal-helper-contract-ssot.md`
       - `ring1-core-provider-scope-ssot.md`
       - `array-map-owner-and-ring-cutover-ssot.md`
       - `de-rust-full-rust-zero-remaining-rust-inventory-ssot.md`
       - `phase-29cc/README.md`
       - `phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md`
       - `phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md`
       - `phase-29cc/29cc-232-kernel-b1-min1-closeout-lock-ssot.md`
  6. `Stage3` gate 追加（landed）
     - bootstrap same-result helper: `tools/selfhost/stage3_same_result_check.sh`
     - build lane compares re-emitted Program/MIR payload snapshots from a known-good seed plus `.artifact_kind`
     - skip-build lane compares an explicit prebuilt pair
- current docs exact leaf:
  - `substrate-capability-ladder-ssot.md`
  - `value-repr-and-abi-manifest-ssot.md`
  - `abi-export-inventory.md`
  - `abi-export-manifest-v0.toml`
  - `handle-cache-metal-helper-contract-ssot.md`
  - `minimal-capability-modules-ssot.md`
  - `minimum-verifier-ssot.md`
  - `raw-array-substrate-ssot.md`
  - `raw-map-substrate-ssot.md`
  - `raw-map-truthful-native-seam-inventory.md`
  - `gc-tls-atomic-capability-ssot.md`
  - `atomic-tls-gc-truthful-native-seam-inventory.md`
  - `stage2-selfhost-and-hako-alloc-ssot.md`
  - `thread-and-tls-capability-ssot.md`
  - `final-metal-split-ssot.md`
  - `rust-kernel-export-surface-strata-ssot.md`
  - `phase-29ct/README.md`
- landed slice:
  - `phase-29ct` V0 ABI export inventory
    - docs-side truth now lives in
      `docs/development/current/main/design/abi-export-inventory.md`
    - current collection/kernel symbols are grouped as:
      `mainline substrate` / `runtime-facade` / `compat-only`
    - `AbiAdapterRegistryBox` is fixed as adapter-default consumer; defaults are manifest-backed and generated
  - `phase-29ct` V0.1 adapter-default manifest + generated shim
    - docs-side truth now lives in
      `docs/development/current/main/design/abi-export-manifest-v0.toml`
    - generated defaults module now lives in
      `lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako`
    - `AbiAdapterRegistryBox` now consumes generated defaults instead of hand-written rows
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
    - live observer slice now exists at:
      `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
      and `MapCoreBox.size_i64` routes through `RawMapCoreBox.entry_count_i64`
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
  - `phase-29ct` I5.1 minimum verifier initialized-range slice
    - second live verifier box is
      `lang/src/runtime/substrate/verifier/initialized_range/initialized_range_core_box.hako`
    - `RawArrayCoreBox.slot_load_i64` now gates through
      `BoundsCoreBox -> InitializedRangeCoreBox -> PtrCoreBox`
    - current readable initialized range is locked to `BufCoreBox.len_i64(handle)`
  - `phase-29ct` I6 RawMap probe/load/store widening
    - `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako` now exposes
      `probe_*` / `slot_load_*` / `slot_store_*`
    - `MapCoreBox.size_i64` still routes through `RawMapCoreBox.entry_count_i64`
    - `MapCoreBox` now routes raw receiver-handle `set/get/has` through
      `RawMapCoreBox` while keeping stateful owner fast paths local
  - `phase-29ct` I7 ownership verifier slice
    - third live verifier box is
      `lang/src/runtime/substrate/verifier/ownership/ownership_core_box.hako`
    - current live subset is
      `ensure_handle_readable_i64` / `ensure_handle_writable_i64` /
      `ensure_any_readable_i64`
    - `RawArrayCoreBox` and `RawMapCoreBox` now gate current raw routes through
      ownership before the deeper substrate backend
  - `phase-29ct` I8 RawMap capacity observer slice
    - `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako` now also exposes
      `cap_i64(handle)`
    - native capacity observer seam is `nyash.map.cap_h`
    - `rehash/tombstone` stay parked until a truthful native seam exists
  - `phase-29ct` I9 RawMap truthful native seam inventory
    - docs-side truth now lives in
      `docs/development/current/main/design/raw-map-truthful-native-seam-inventory.md`
    - current `RawMap` widening is constrained by the `HashMap` backend truth
    - live rows remain:
      `entry_count_h` / `cap_h` / `probe_*` / `slot_load_*` / `slot_store_*`
    - `rehash/tombstone/bucket_*` remain parked by design
  - `phase-29ct` I10 Atomic/TLS/GC truthful native seam inventory
    - docs-side truth now lives in
      `docs/development/current/main/design/atomic-tls-gc-truthful-native-seam-inventory.md`
    - current widening is seam-first:
      inventory -> `gc` first live row -> helper-shaped `tls/atomic` proof rows
    - `nyash.gc.barrier_write` is the current truthful live seam
  - `phase-29ct` I11 GC first live slice
    - `lang/src/runtime/substrate/gc/gc_core_box.hako` now exposes
      `write_barrier_i64(handle_or_ptr)`
    - current native seam is `nyash.gc.barrier_write`
    - generic `atomic/tls` remain parked
  - `phase-29ct` I12 stage/selfhost + TLS end-state lock
    - docs-side truth now also lives in
      `docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md`
      and `docs/development/current/main/design/thread-and-tls-capability-ssot.md`
    - `stage0/stage1/stage2/stage3` and `owner/substrate` are now read as separate axes
    - final library layering is fixed as:
      `hako_core / hako_alloc / hako_std`
  - `phase-29ct` I13 helper-shaped TLS/atomic proof rows
    - `lang/src/runtime/substrate/tls/tls_core_box.hako` now exposes
      `last_error_text_h()`
    - `lang/src/runtime/substrate/atomic/atomic_core_box.hako` now exposes
      `fence_i64()`
    - current live `atomic/tls` rows are intentionally helper-shaped, not final-form generic APIs
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
- `phase-29ct`: substrate capability ladder = stop-line reached (docs-only)

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
