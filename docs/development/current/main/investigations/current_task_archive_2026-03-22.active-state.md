# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-20
Scope: repo root の再起動入口。詳細ログは `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で `Current Blocker` と `next fixed order` に到達する。
- 本ファイルは薄い入口に保ち、長文履歴はアーカイブへ逃がす。
- kernel migration lane の Next は `docs/development/current/main/phases/phase-29cm/README.md` を単一正本に固定する。
- de-rust runtime lane の Next は `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md` を単一正本に固定する。
- VM `.hako` migration / BoxShape の Next は `docs/development/current/main/phases/phase-29y/83-VM-S0-REFACTOR-OUTSOURCE-INSTRUCTIONS.md` を parked strict-polish 読みの単一正本に固定する。
- VM boxcall contract の Next は `docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md` を単一正本に固定する。
- `0rust` buildability contract の Next は `docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md` を単一正本に固定する。
- `stage0/stage1/stage2+` と `owner/substrate` の軸分離は `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md` を単一正本に固定する。
- backend-zero fixed order / buildability gate の Next は `docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md` を単一正本に固定する。
- repo physical cleanup / BoxShape cleanup の Next は `docs/development/current/main/phases/phase-29cr/README.md` を単一正本に固定する。P0 root hygiene の first batch は landed（root scratch/docs archive move + `*.err` / `*.backup*` ignore）で、次は P1 `CURRENT_TASK` slim。
- stage axis: `stage0` Rust bootstrap keep / `stage1` proof / `stage2+` daily mainline
- owner axis practical end-state: `.hako` owns kernel meaning/policy/control, Rust stays bootstrap/recovery/raw substrate, and LLVM remains the primary backend substrate

## End-State Checklist

- [ ] compiler authority zero
- [ ] kernel authority zero
- [ ] backend-zero daily owner cutover
- [ ] substrate reconsideration
  - LLVM remains the primary backend substrate unless another SSOT explicitly reassigns it

## VM Migration Inventory

- already split from `mir_vm_s0.hako`:
  - `lang/src/vm/boxes/mir_vm_s0_json_scan.hako`
  - `lang/src/vm/boxes/mir_vm_s0_state_ops.hako`
  - `lang/src/vm/boxes/mir_vm_s0_reg_utils.hako`
  - `lang/src/vm/boxes/mir_vm_s0_args_phi.hako`
  - `lang/src/vm/boxes/mir_vm_s0_block_loc.hako`
  - `lang/src/vm/boxes/mir_vm_s0_lifecycle_ops.hako`
  - `lang/src/vm/boxes/mir_vm_s0_call_exec.hako`
  - `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`
  - `lang/src/vm/boxes/mir_vm_s0_codegen.hako`
  - `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako`
  - `lang/src/vm/boxes/mir_vm_s0_exec_dispatch.hako`
  - `lang/src/vm/boxes/mir_vm_s0_block_runner.hako`
- `mir_vm_s0.hako` is now a thin facade; the remaining VM execution logic is owned by `mir_vm_s0_exec_dispatch.hako`, `mir_vm_s0_call_exec.hako`, `mir_vm_s0_boxcall_builtin.hako`, `mir_vm_s0_boxcall_exec.hako`, `mir_vm_s0_codegen.hako`, and `mir_vm_s0_block_runner.hako`
- done-enough reading:
  1. `mir_vm_s0.hako` facade-only shape is landed
  2. execution ownership is already localized in `mir_vm_s0_exec_dispatch.hako` / `mir_vm_s0_call_exec.hako` / `mir_vm_s0_boxcall_builtin.hako` / `mir_vm_s0_boxcall_exec.hako` / `mir_vm_s0_codegen.hako` / `mir_vm_s0_block_runner.hako`
  3. entry / module wiring is green
  4. remaining work is strict-polish only; do not reopen unless a new exact blocker appears

## VM Migration Quick Entry

1. `docs/development/current/main/phases/phase-29y/80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md`
2. `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
3. `docs/development/current/main/phases/phase-29y/82-VM-HAKO-BOXCALL-CONTRACT-SSOT.md`
4. `docs/development/current/main/phases/phase-29y/83-VM-S0-REFACTOR-OUTSOURCE-INSTRUCTIONS.md`

## Active Slice

- Current blocker:
  - `dev_gate portability` is green again; the `phase21_5_perf_kilo_text_concat_contract_vm.sh` residual route slice is closed after `ArrayBox` string-element propagation + boxcall set-route alignment, so the last blocker is no longer the `nyash.any.length_h` route
  - the macOS portability helper move remains committed and green; `src/host_providers/llvm_codegen.rs` still centralizes FFI library candidate resolution, and `tools/checks/macos_portability_guard.sh` remains green
  - no collection/runtime blocker remains, and the smoke runner now has a first-class suite manifest contract; inventory is suite-aware, the first twenty-nine semantic splits have landed, `phase29ck_boundary` now lives under `integration/phase29ck_boundary/{entry,string,runtime_data}/`, `vm_hako_caps` now lives under `integration/vm_hako_caps/{app1,args,compare,env,file,gate,lib,mapbox,misc,open_handle_phi,select_emit}/`, the first eight `phase29cc_wsm` splits (`g3_canvas`, `g2_browser`, `g4`, `p10`, `p5`, `p6`, `p7`, and `p8`) now live under `integration/phase29cc_wsm/{g3_canvas,g2_browser,g4,p10,p5,p6,p7,p8}/`, `phase29cc/plg_hm1` now lives under `integration/phase29cc/plg_hm1/`, `phase29x/vm_hako` now lives under `integration/phase29x/vm_hako/`, `phase29x/derust` now lives under `integration/phase29x/derust/`, `phase29x/observability` now lives under `integration/phase29x/observability/`, `phase29y/hako/emit_mir` now lives under `integration/phase29y/hako/emit_mir/`, `phase21_5/perf/{chip8,kilo}` now live under `integration/phase21_5/perf/{chip8,kilo}/`, and `phase21_5/perf/numeric` now lives under `integration/phase21_5/perf/numeric/`
  - `phase-29cq` smoke split is parked after the landed semantic families above; it is no longer the active blocker lane
  - axis lock: read the current state through `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
  - lane B fast-CI blocker is closed in two exact steps:
    - `29bq-116`: Rust `--emit-mir-json` now serializes `main` before helper functions
    - `29bq-117`: llvmlite harness now accepts `ArrayBox.birth()` as the initializer no-op after `newbox ArrayBox`
  - the adjacent lane C / `.hako VM` (`vm-hako`) map blocker sweep is now closed through `RVP-C28`; no current vm-hako map blocker remains, and phase-29y is parked until a new exact blocker appears
  - regression repair pinned: `RVP-C02 args.length()` no longer treats missing `handle_regs/file_boxes` entries as visible `[map/missing] ...` text; runtime state maps now use presence-aware storage reads
  - collection owner shift reached the done-enough stop line for this phase; `.hako` owns visible collection semantics while Rust still owns the raw substrate / plugin ABI path
  - phase stop-line here means phase-local owner progress, not end-state completion across the stage axis or the owner axis
  - raw substrate perf is parked until the remaining daily collection keep is either removed or explicitly accepted; the `array` read-seam keep remains the last accepted perf slice at `ny_aot_ms=43`
  - immediate write-side probes were rejected and reverted:
    - dedicated `handle_helpers` i64 write helper: `43 -> 47 ms`
    - `ArrayBox::try_set_index_i64_integer()` cold-split: `43 -> 48 ms`
    - `with_array_box` cache-hit inline probe: fresh recheck stayed at `46 ms`, and microasm still concentrated on `array_slot_store_i64` closure + `LocalKey::with`
  - `B1a` landed: the daily `.hako` path now uses `nyash.array.slot_len_h`, while `nyash.array.len_h` remains compat-only
  - `B1b` landed: the daily `.hako` path and arrayish runtime-data mono-route now use `nyash.array.slot_append_hh`, while `nyash.array.push_hh` remains compat-only
  - `B1c` landed: the daily `.hako` map observer path now uses `nyash.map.entry_count_h`, while `nyash.map.size_h` remains compat-only
  - `B1d1` landed: `nyash.array.slot_append_hh` now appends through `ArrayBox.slot_append_box_raw(...)`; compat append routes share the same raw helper instead of calling visible `push()`
  - `B1d2` landed: `nyash.array.slot_store_hii` and runtime-data array set now store through `ArrayBox.slot_store_*_raw(...)`; append-at-end/rebox behavior is preserved, but the visible `try_set_index_i64*()` methods are no longer the substrate seam
  - `B1e` landed: `nyash.map.slot_* / probe_*` and `nyash.map.entry_count_h` now execute through `MapBox.{get_opt_key_str,insert_key_str,contains_key_str,entry_count_i64}(...)` instead of visible `get_opt/set/has/size`
  - worker stop-line inventory:
    - the new `MapBox` raw key-string helpers are good enough as the kernel-side raw seam for this slice
    - but the collection boundary is still not closed, because active daily lowering/runtime-data paths still cross method-shaped collection exports
  - the next collection task is exact `B1` taskization:
    1. `B1f` landed: active daily `collections_hot.hako` rewrites now target raw seams for array `get/push` and map `get/set/has`; `array set` stays on the current route until a raw non-i64-safe write seam is accepted
    2. `B1g` landed: active llvm-py lowering now uses raw seams where they already exist (`array push`, `array i64 get`, `map get/set/has`); the remaining array keep was deferred to `B1i/B1j`
    3. `B1h` landed: `runtime_data_map_route.rs` now delegates map behavior through accepted `map_slot_load_any` / `map_slot_store_any` / `map_probe_contains_any`
    4. `B1i` first slice landed: active lowering now routes array non-i64 `get/has` and non-i64 `set` through `nyash.runtime_data.*`, while the i64 raw/near-raw routes stay `slot_load_hi` / `set_hih` / `set_hii`
    5. `B1j` landed: the remaining i64-key array set path is now an explicit accepted keep; `nyash.array.set_hii` stays i64/i64-specialized and `nyash.array.set_hih` stays the i64-key + handle/any-value fallback, with no new `slot_store_hih`
  - build-freshness note is now pinned: after a new kernel export lands on the AOT boundary path, refresh release-side artifacts before link/pure smokes; stale pure-link failures must fail fast on missing staticlib symbols instead of relying on manual rebuild memory
  - `RuntimeDataBox` has no active code task now; keep it facade-only and reopen only on an exact protocol/dispatch bug
  - `crates/nyash_kernel/src/plugin/array_index_helpers.rs` / `array_route_helpers.rs` are now thin wrappers and should not be treated as the primary P1 edit target
- Later cleanup (not this slice):
  - rename `apps/tests/vm_hako_caps/mapbox_set_block_min.hako` after the current RVP wave settles
  - factor `filter_noise || true` handling into a shared smoke helper instead of per-smoke local glue
  - smoke hygiene: `tools/smokes/v2/profiles/integration/apps/*.sh` is now dense enough that new scripts should be added only for `1 blocker = 1 pin`; after each lane reaches stop line, fold day-to-day execution back to gate packs and inventory scripts instead of keeping every pin in the daily read path
  - smoke hygiene: keep `tools/checks/dev_gate.sh` / lane gate packs as the daily entry, and treat single-purpose app smokes as evidence pins or blocker probes
  - smoke hygiene: use `tools/checks/smoke_inventory_report.sh` at milestones instead of ad-hoc manual pruning
  - smoke hygiene: `tools/smokes/v2/run.sh` now discovery-prunes `archive/lib/tmp/fixtures`; treat those names as non-live support buckets and keep new semantic growth under `profile -> domain -> intent`
  - smoke hygiene: suite manifests live under `tools/smokes/v2/suites/<profile>/<suite>.txt`; keep them small, active-only, and human-meaningful
  - smoke hygiene: first future split families have landed at `tools/smokes/v2/profiles/integration/rc_gc_alignment/`, `tools/smokes/v2/profiles/integration/json/`, `tools/smokes/v2/profiles/integration/mir_shape/`, and `tools/smokes/v2/profiles/integration/ring1_providers/`; `phase29ck_boundary` has now been split into `tools/smokes/v2/profiles/integration/phase29ck_boundary/{entry,string,runtime_data}/`, `vm_hako_caps` has now been split into `tools/smokes/v2/profiles/integration/vm_hako_caps/{app1,args,compare,env,file,gate,lib,mapbox,misc,open_handle_phi,select_emit}/`, `phase29cc/plg_hm1`, `phase29x/vm_hako`, `phase29x/derust`, `phase29x/observability`, `phase29y/hako/emit_mir`, `phase21_5/perf/{chip8,kilo}`, and `phase21_5/perf/numeric` now live under `tools/smokes/v2/profiles/integration/{phase29cc/plg_hm1,phase29x/vm_hako,phase29x/derust,phase29x/observability,phase29y/hako/emit_mir,phase21_5/perf/{chip8,kilo,numeric}}/`; continue splitting the remaining active families out of `tools/smokes/v2/profiles/integration/apps/` by domain, keeping the bundle root empty of new live `phase21_5/perf/apps` scripts
  - smoke hygiene: inventory now reports suite coverage; use the suite-aware report before semantic path splits
- Next exact files:
  - `crates/nyash_kernel/src/plugin/array_slot_store.rs`
  - `crates/nyash_kernel/src/plugin/handle_helpers.rs`
  - `src/boxes/array/mod.rs`
  - `tools/perf/bench_micro_c_vs_aot_stat.sh`
  - `tools/perf/bench_micro_aot_asm.sh`
  - `docs/development/current/main/design/perf-optimization-method-ssot.md`
  - `docs/development/current/main/design/optimization-tag-flow-ssot.md`
  - `docs/development/current/main/phases/phase-29cm/README.md`
- Execution checklist:
  - `[x]` VM lane reached done-enough stop line
  - `[x]` backend-zero reached stop line for the current owner/compat keep waves
  - `[x]` optimization tag/knob inventory is pinned by reach (`pre-boundary` / `boundary` / `keep-lane-only` / `perf-only`)
  - `[x]` `array_getset` proved the remaining method-shaped collection hotspot still lives in Rust substrate
  - `[x]` reopen kernel authority lane as collection owner cutover (`array -> map -> runtime_data cleanup`)
  - `[x]` A1 first slice landed: `ArrayCoreBox` owns `ArrayBox` push/get/set/size fallback routing and `mir_call_v1_handler.hako` no longer carries Array-specific size/push branches
  - `[x]` A2 first slice landed: Rust `array` helper ownership is split into raw `slot_load` / `slot_store` modules while legacy method-shaped helper names remain thin wrappers
  - `[x]` A3 first slice landed: `ArrayCoreBox.get_i64/set_i64` now target raw `slot_load/slot_store` exports while legacy `get_hi/set_hii` stay compat-only
  - `[x]` array first wave reached the current stop line: `len` now sits on `nyash.array.slot_len_h`, and `push` now sits on `nyash.array.slot_append_hh`; hidden append/write residue still remains below those raw names
  - `[ ]` complete the end-state `ArrayBox` owner migration below the remaining transitional raw boundary
  - `[ ]` complete the end-state `MapBox` owner migration below the remaining transitional raw boundary
  - `[x]` M1 first slice landed: `MapCoreBox` is the single handler-side visible owner frontier for `MapBox.{set,get,has,size/len/length}` and `mir_call_v1_handler.hako` no longer carries inline MapBox set fallback logic
  - `[x]` M1 second slice landed: Rust `map` helper ownership is split into raw `slot_load` / `slot_store` / `probe` modules while legacy `nyash.map.{get,set,has}_*` exports remain thin compatibility wrappers
  - `[x]` M1 third slice landed: `map_state_core_box.hako` now owns vm-hako-visible `MapBox.{set,get,has,getField,setField,delete,keys,clear}` stateful routing and `mir_vm_s0_boxcall_builtin.hako` only delegates
  - `[x]` R1 first slice landed: `runtime_data.rs` is now a dispatch shell over `runtime_data_array_route.rs` / `runtime_data_map_route.rs`, while `RuntimeDataBox` remains protocol/facade-only and keeps the same `nyash.runtime_data.*` export contract
  - `[x]` R1 second slice landed: `runtime_data_core_box.hako` now owns its own arg-decode/ABI-dispatch helpers and `mir_call_v1_handler.hako` treats `RuntimeDataBox` as a single delegated branch
  - `[x]` phase-29cm minimum acceptance is green (`phase29cc_runtime_v0_adapter_fixtures_vm`, `phase29cc_runtime_v0_abi_slice_guard`, `array_length_vm`, `map_basic_get_set_vm`, `map_len_size_vm`, `ring1_array_provider_vm`, `ring1_map_provider_vm`, `phase29x_runtime_data_dispatch_llvm_e2e_vm`)
  - `[x]` collection owner shift reached the done-enough stop line for `array` / `map` / `runtime_data`
  - `[x]` `.hako` ring1 is now the visible owner frontier for `ArrayBox` / `MapBox`, and `RuntimeDataBox` is facade-only
  - `[ ]` deepen the boundary below the remaining hidden raw-named residue (`nyash.array.slot_append_hh` / `nyash.array.slot_store_hii` / `nyash.map.slot_*` / `nyash.map.probe_*`)
  - `[ ]` reopen raw substrate perf only after the deeper boundary is fixed or the transitional exports are explicitly accepted as the long-term substrate cut
  - `[x]` phase-29cq first slice: introduce suite manifests as the smoke execution contract (`--profile` stays compatible, `--suite` is opt-in)
  - `[x]` phase-29cq first slice: seed integration suites (`presubmit`, `collection-core`, `vm-hako-core`, `selfhost-core`, `joinir-bq`)
  - `[x]` phase-29cq first slice: keep `integration/apps` new additions frozen while suite manifests stabilize
  - `[x]` phase-29cq second slice: make `tools/checks/smoke_inventory_report.sh` suite-aware
  - `[x]` phase-29cq first semantic split: move `rc_gc_alignment_*` out of `integration/apps` into `integration/rc_gc_alignment/`
  - `[x]` phase-29cq second semantic split: move `json_*` out of `integration/apps` into `integration/json/`
  - `[x]` phase-29cq third semantic split: move `mir_shape_guard` out of `integration/apps` into `integration/mir_shape/`
  - `[x]` phase-29cq fourth semantic split: move `ring1_{array,console,map,path}_provider` out of `integration/apps` into `integration/ring1_providers/`
  - `[x]` phase-29cq fifth semantic split: move `phase29ck_boundary` out of `integration/apps` into `integration/phase29ck_boundary/{entry,string,runtime_data}/`
  - `[x]` phase-29cq sixth semantic split: move `vm_hako_caps` out of `integration/apps` into `integration/vm_hako_caps/{app1,args,compare,env,file,gate,lib,mapbox,misc,open_handle_phi,select_emit}/`
  - `[x]` phase-29cq ongoing split: move `phase29cc_wsm/g3_canvas` out of `integration/apps` into `integration/phase29cc_wsm/g3_canvas/`
  - `[x]` phase-29cq ongoing split: move `phase29cc_wsm/g2_browser` out of `integration/apps` into `integration/phase29cc_wsm/g2_browser/`
  - `[x]` phase-29cq ongoing split: move `phase29cc_wsm/g4` out of `integration/apps` into `integration/phase29cc_wsm/g4/`
  - `[x]` phase-29cq ongoing split: move `phase29cc_wsm/p10` out of `integration/apps` into `integration/phase29cc_wsm/p10/`
  - `[x]` phase-29cq ongoing split: continue splitting the remaining `phase29cc_wsm` families by semantic domain (next family: `phase29cc_wsm/p5`)
  - `[x]` phase-29cq ongoing split: continue splitting the remaining `phase29cc_wsm` families by semantic domain (next family: `phase29cc_wsm/p6`)
  - `[x]` phase-29cq ongoing split: continue splitting the remaining `phase29cc_wsm` families by semantic domain (next family: `phase29cc_wsm/p7`)
  - `[ ]` phase-29cq ongoing split: continue splitting the remaining active families by semantic domain (next family: `phase29x` residual family, starting with `derust`)
  - `[x]` RVP-C16 first vm-hako blocker is closed: `newbox(MapBox)` is accepted in subset-check and pinned by `vm_hako_caps_mapbox_newbox_ported_vm.sh`
  - `[x]` RVP-C17 is closed: `MapBox.set(key,value)` now clears subset/runtime args>1 blockers and is pinned by `vm_hako_caps_mapbox_set_ported_vm.sh`
  - `[x]` RVP-C18 is closed: `MapBox.size()` now completes in vm-hako and is pinned by `vm_hako_caps_mapbox_size_ported_vm.sh`
  - `[x]` RVP-C19 is closed: `MapBox.get(key)` now returns the stored scalar value and is pinned by `vm_hako_caps_mapbox_get_ported_vm.sh`
  - `[x]` lane B blocker `29bq-116` is closed: Rust `--emit-mir-json` now serializes `main` first
  - `[x]` lane B blocker `29bq-117` is closed: llvmlite harness accepts `ArrayBox.birth()` as the initializer no-op after `newbox ArrayBox`
  - `[x]` fast-smoke EXE trio is green again: `ternary_basic -> 10`, `ternary_nested -> 50`, `peek_expr_block -> 1`
  - `[x]` RVP-C20 is closed: `MapBox.has(key)` now preserves visible bool parity (`true/false`) and is pinned by `vm_hako_caps_mapbox_has_ported_vm.sh`
  - `[x]` RVP-C21 is closed: `MapBox.delete(key)` now removes presence/value state, preserves `has/size` parity, and is pinned by `vm_hako_caps_mapbox_delete_ported_vm.sh`
  - `[x]` RVP-C22 is closed: `MapBox.keys()` now returns an ArrayBox-like token whose `size()` matches the map size, and is pinned by `vm_hako_caps_mapbox_keys_ported_vm.sh`
  - `[x]` RVP-C23 is closed: `MapBox.clear()` now resets visible `size/has/keys` state and is pinned by `vm_hako_caps_mapbox_clear_ported_vm.sh`
  - `[x]` RVP-C24 is closed: `MapBox.get(missing-key)` now returns the stable `[map/missing] Key not found: <key>` text and is pinned by `vm_hako_caps_mapbox_get_missing_ported_vm.sh`
  - `[x]` RVP-C25 is closed: `MapBox.get(non-string key)` now returns the stable `[map/bad-key] key must be string` text and is pinned by `vm_hako_caps_mapbox_get_bad_key_ported_vm.sh`
  - `[x]` RVP-C26 is closed: `MapBox.set(non-string key, value)` now returns the stable `[map/bad-key] key must be string` text and is pinned by `vm_hako_caps_mapbox_set_bad_key_ported_vm.sh`
  - `[x]` RVP-C27 is closed: `MapBox.getField(non-string key)` now returns the stable `[map/bad-key] field name must be string` text and is pinned by `vm_hako_caps_mapbox_getfield_bad_key_ported_vm.sh`
  - `[x]` RVP-C28 is closed: `MapBox.setField(non-string key, value)` now returns the stable `[map/bad-key] field name must be string` text and is pinned by `vm_hako_caps_mapbox_setfield_bad_key_ported_vm.sh`
  - `[x]` lane C map bad-key field sweep reached the stop line; reopen only if a new exact vm-hako blocker appears
  - `[x]` `RuntimeDataBox` facade-only stop line reached
    - reopen only on an exact protocol/dispatch blocker; do not reopen for collection-owner growth
  - `[x]` backend-zero current owner cutover is closed enough for handoff
  - `[x]` `BackendRecipeBox` route-profile validation no longer relies on dead recipe-label helpers
  - `[x]` inventory the first compat keep reduction slice without mixing bootstrap keep reduction
  - `[x]` first compat keep reduction slice is fixed to `src/host_providers/llvm_codegen.rs` / `src/host_providers/llvm_codegen/route.rs`
  - `[x]` `route.rs` shared keep compile setup is factored behind `compile_via_capi_keep_internal(...)`
  - `[x]` `src/host_providers/llvm_codegen.rs` is already thin enough; no further code slice stays in that file
  - `[x]` `crates/nyash-llvm-compiler/src/boundary_driver.rs` is facade-only; FFI plumbing moved into `boundary_driver_ffi.rs`
  - `[x]` `crates/nyash-llvm-compiler/src/boundary_driver_ffi.rs` has no further safe thinning slice
  - `[x]` keep `compile_symbol_for_keep_recipe()` generic default parked; do not reopen it during collection owner cutover
- Stop condition:
  - collection cutover order stays `array -> map -> runtime_data cleanup`
  - `string` remains at stop line unless a new exact blocker appears
  - `numeric` remains parked until collection owner cutover settles or a new exact blocker appears
  - do not move collection semantics into `ring0`
  - Rust `stage0` bootstrap / recovery route remains green
  - raw substrate micro-opt stays parked until method-shaped `array` / `map` verbs leave Rust ownership or an exact blocker says otherwise
- Do not mix:
  - VM strict-polish reopening without a new exact blocker
  - Rust substrate micro-optimization before `array` owner cutover
  - `runtime_data` owner growth
  - C shim transport micro-splitting
  - `native_driver.rs` bootstrap keep reduction

## Bootstrap Check

- observed state: `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` now gets past the C ABI rebuild seam and the bridge-first MIR build, and the reduced artifact itself is treated as runnable bootstrap output while payload proof stays on the stage0 bootstrap route
- exact blocker: none for the bootstrap route repair; the legacy `NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_PROGRAM_JSON=1 ... target/selfhost/hakorune.stage1_cli` probe is diagnostics-only
- current reduced source: the stage1-cli artifact is no longer treated as the payload-emitting contract; keep the bootstrap route proof and the runnable artifact check separate, and use the stage0 bootstrap route for payload materialization
- the bootstrap capability probe is now single-sourced in `stage1_contract_verify_stage1_cli_bootstrap_capability()`
- the legacy env payload probe is a diagnostics lane; do not mix it with the `.hako` authoring slice above
- next phase: `docs/development/current/main/phases/phase-29cp/README.md`
- next exact files:
  - `tools/selfhost/lib/stage1_contract.sh`
  - `tools/selfhost/lib/identity_routes.sh`
  - `tools/selfhost/build_stage1.sh`
  - `tools/selfhost/README.md`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stage1_contract_smoke_vm.sh`
  - `docs/development/current/main/phases/phase-29cp/README.md`

## Current Priority

- immediate: reopen `P1` now that the remaining daily array i64-key set keep is explicitly accepted as the long-term substrate cut for this slice
- second: keep boundary-deepen work parked unless a new exact collection blocker appears
- side-fix complete: backend-zero macOS portability slice is green; `src/host_providers/llvm_codegen.rs` centralizes FFI library candidate resolution
- side-fix complete: lane B fast-smoke blocker is fixed by `29bq-116` + `29bq-117`
- first: the smoke split backlog stays parked after `phase29x/derust` and `phase29x/observability`
- side-plan prepared: repo physical cleanup / BoxShape cleanup is pinned at `phase-29cr`; P0 root hygiene first batch is landed, and the next exact slice is P1 `CURRENT_TASK` slim. Do not mix it into the current exact perf/runtime slice.
- third: keep `RuntimeDataBox` as protocol / facade only; do not reopen owner growth
- note: this is a `.hako VM` capability blocker, not a Rust VM blocker
- parked: boundary-deepen work stays paused unless a new exact collection blocker appears
- parked: backend-zero compat keep reduction is at stop line; do not reopen it unless a new exact blocker appears
- parked: VM strict-polish is no longer active; reopen only if a new exact blocker appears
- parked: bootstrap keep reduction stays parked unless compat keep lanes reopen
- parked: smoke split backlog is no longer the daily read path; `phase29x/derust` and `phase29x/observability` are landed, and further phase29x residual splits are inventory-only until kernel migration is explicitly reopened
- operational reading: `stage0` Rust bootstrap remains first-build / recovery lane; `stage2+` mainline stays separate
- parallel `.hako` authoring lane: `lang/src/runtime/kernel/string/search.hako` remains at stop line; reopen only if a new exact blocker appears
- `phase-29cl` by_name mainline callers are already zero; remaining work is compat/archive closeout only
- directory rule: keep `Stage1/Stage2` as artifact/proof names; Rust physical split stays `src/stage1/**` vs `src/runner/stage1_bridge/**`, do not create a mirrored `src/stage2/`

## Implementation Order (Array First)

1. `A1: Array semantics lock`
   - `.hako` owners:
     - `lang/src/runtime/collections/array_core_box.hako`
     - `lang/src/runtime/collections/array_state_core_box.hako`
   - fix the visible owner for:
      - `ArrayBox.{get,set,push,len,length,size}`
      - bounds policy
      - normalization decisions
      - visible fallback/error contract
   - first landed slice:
      - `ArrayCoreBox` owns `push/get/set/size` fallback routing
      - `mir_call_v1_handler.hako` delegates `ArrayBox` semantics instead of owning them
2. `A2: Array raw substrate contract`
   - SSOT:
     - `docs/development/current/main/design/collection-raw-substrate-contract-ssot.md`
   - Rust owners:
     - `crates/nyash_kernel/src/plugin/array.rs`
     - `crates/nyash_kernel/src/plugin/array_index_helpers.rs`
     - `crates/nyash_kernel/src/plugin/array_route_helpers.rs`
     - `crates/nyash_kernel/src/plugin/handle_helpers.rs`
   - demote to raw verbs only:
     - `slot_load`
     - `slot_store`
     - `reserve/grow`
     - cache/downcast/layout
   - first landed slice:
      - `crates/nyash_kernel/src/plugin/array_slot_load.rs`
      - `crates/nyash_kernel/src/plugin/array_slot_store.rs`
      - `array_index_helpers.rs` / `array_route_helpers.rs` are now thin wrappers over raw slot helpers
3. `A3: Array retarget`
   - point `.hako` array owner at raw substrate verbs only
   - remove method-shaped Rust ownership from the daily path
   - first landed slice:
      - `ArrayCoreBox.get_i64/set_i64` now target raw `nyash.array.slot_load_hi` / `nyash.array.slot_store_hii`
      - legacy `nyash.array.get_hi/set_hii` remain as compat shell
4. `M1: Map semantics + substrate split`
   - repeat `A1 -> A3` for `MapBox`
5. `R1: RuntimeData cleanup`
   - keep `RuntimeDataBox` as protocol / facade only
   - do not absorb array/map semantics
6. `B1: Deeper collection boundary before perf`
   - landed:
     - `nyash.array.len_h` -> `nyash.array.slot_len_h` on the daily `.hako` path
     - `nyash.array.push_hh` -> `nyash.array.slot_append_hh` on the daily `.hako` path and arrayish runtime-data mono-route
     - `nyash.map.size_h` -> `nyash.map.entry_count_h` on the daily `.hako` path
7. `P1: Raw substrate perf reopen`
   - active again once `B1` is fixed or the last daily keep is explicitly accepted as the long-term substrate boundary

## Remaining Migration Checklist

- [x] VM `.hako` migration lane reached done-enough stop line
  - [x] helper split baseline already in place (`json_scan` / `state_ops` / `reg_utils` / `args_phi` / `block_loc` / `lifecycle_ops` / `boxcall_exec`)
  - [x] responsibility inventory freeze for `mir_vm_s0.hako`
  - [x] split remaining execution helpers out of `mir_vm_s0.hako`（`mir_vm_s0_exec_dispatch.hako`）
  - [x] split block runner helpers out of `mir_vm_s0.hako`（`mir_vm_s0_block_runner.hako`）
  - [x] unify duplicated `phi` / `branch` / `jump` control-flow helpers inside `mir_vm_s0_block_runner.hako`
  - [x] centralize register payload transfer for `phi` / `copy` in `mir_vm_s0_state_ops.hako`
  - [x] split call / externcall / newbox / ret helpers out of `mir_vm_s0_exec_dispatch.hako`（`mir_vm_s0_call_exec.hako`）
  - [x] split builtin file/string/array boxcall helpers out of `mir_vm_s0_boxcall_exec.hako`（`mir_vm_s0_boxcall_builtin.hako`）
  - [x] thin entry / module wiring（`mini_vm_s0_entry.hako` now binds directly to `MirVmS0BlockRunnerBox.run_min(...)`）
  - [x] remove legacy top-array recovery from `mir_vm_s0_block_runner.hako`（structured payload only; unsupported payload shapes fail fast）
  - [x] split boxcall routing out of `mir_vm_s0_exec_dispatch.hako`（`mir_vm_s0_boxcall_exec.hako`）
  - [x] split backend codegen apply helpers out of `mir_vm_s0_boxcall_exec.hako`（`mir_vm_s0_codegen.hako`）
  - [x] thin `mir_vm_s0.hako` to orchestration / dispatch glue
  - [x] keep `vm_hako_caps/gate/phase29y_vm_hako_caps_gate_vm.sh` / `phase29y_no_compat_mainline_vm.sh` / `phase29y_lane_gate_vm.sh` green
  - [x] park strict-polish unless a new exact blocker appears
- [x] bootstrap check / `phase-29cp`
  - [x] stage0 bootstrap route materializes Program(JSON v0) / MIR(JSON v0)
  - [x] reduced `stage1-cli` artifact is runnable bootstrap output
  - [x] `tools/selfhost/lib/stage1_contract.sh`
  - [x] `tools/selfhost/lib/identity_routes.sh`
  - [x] `tools/selfhost/build_stage1.sh`
  - [x] `tools/selfhost/README.md`
  - [x] `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stage1_contract_smoke_vm.sh`
  - [x] `docs/development/current/main/phases/phase-29cp/README.md`
- [ ] kernel collection owner cutover
  - [x] `string` lane stopped at helper extraction; no widening unless a new exact blocker appears
  - [ ] `array` owner cutover to `.hako` ring1 collection core
  - [ ] `map` owner cutover to `.hako` ring1 collection core
  - [ ] `runtime_data` cleanup to protocol / facade only
  - [x] `numeric` remains parked as a narrow pilot outside the collection owner order
- [x] backend-zero stop line / handoff
  - [x] current owner cutover is closed enough for handoff
  - [x] compat keep reduction inventory / first exact slice
    - [x] `src/host_providers/llvm_codegen.rs`
    - [x] `src/host_providers/llvm_codegen/route.rs`
    - [x] `route.rs` shared keep compile setup is factored behind `compile_via_capi_keep_internal(...)`
    - [x] `crates/nyash-llvm-compiler/src/boundary_driver.rs` is facade-only; FFI plumbing moved into `boundary_driver_ffi.rs`
  - [x] bootstrap keep reduction stays parked while collection owner cutover is active

## Restart Handoff (2026-03-19)

- last landed:
  - `bf800ec79` `backend-zero: enforce route profile ownership`
  - `BackendRecipeBox.compile_route_profile(...)` now exact-validates the canonical owner names and route evidence before returning the profile
- current stable shape:
  - `.hako` route/profile owner is visible and enforced
  - Rust build/bootstrap route remains runnable
  - buildability is a gate contract, not an authority
- next code slice after restart:
  - inventory / explicitize upstream `env.codegen.emit_object` / `env.codegen.compile_json_path` callers first
  - `lang/src/runtime/host/host_facade_box.hako`
  - `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako`
  - `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
  - `src/backend/mir_interpreter/handlers/extern_provider.rs`
  - remove the remaining implicit route-default synthesis in `src/host_providers/llvm_codegen/route.rs` only after caller proof
- current session completion:
  - `boundary_default_object_opts(...)` is now transport-only; `route.rs` no longer synthesizes a hidden `pure-first/harness` route, and the two explicit caller sites now set `compile_recipe=pure-first` / `compat_replay=harness` themselves
  - caller-side explicitization is now also mirrored in `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` and `src/backend/mir_interpreter/handlers/extern_provider.rs`, so `route.rs` can stop reading env defaults for daily callers
  - `src/config/env/llvm_provider_flags.rs::backend_codegen_request_defaults(...)` is now the single shared helper for backend recipe env fallback at compat bridges
  - `src/host_providers/llvm_codegen/route.rs` now reads explicit `Opts` only; the remaining generic symbol is parked as compat-only, not a blocker
  - `crates/nyash-llvm-compiler/src/boundary_driver.rs` no longer injects boundary-local recipe/replay env defaults; it now just calls the explicit pure-first export and mirrors caller env when needed for link-side plumbing
  - `crates/nyash-llvm-compiler/src/link_driver.rs` now requires an explicit `--nyrt <DIR>` for `Harness` / `Native` exe linking instead of synthesizing a default search dir
  - `crates/nyash-llvm-compiler/README.md` and `src/main.rs` now describe that keep-lane exe linking contract explicitly
  - `crates/nyash-llvm-compiler/src/native_driver.rs` now delegates MIR-to-IR construction into `src/native_ir.rs`, so the native bootstrap lane is mostly orchestration plus object emission
  - `src/runner/modes/common_util/exec.rs` and `tools/build_llvm.sh` no longer route `NYASH_LLVM_BACKEND=native`; native replay is now direct `ny-llvmc --driver native` canary only
  - `src/stage1/program_json_v0.rs` now inlines the future-retire bridge error-prefix helper and drops the legacy test-only `source_to_program_json_v0(...)` alias, so `program_json_v0/bridge_shim.rs` is gone and the remaining bootstrap keep is smaller
  - `CodegenBridgeBox.array_arg_or_null(...)` is now the shared payload-normalization helper for legacy env.codegen args, and the dead local copies were removed from `HostFacadeBox` / `MirVmS0BoxcallExecBox`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` / `build_surrogate.rs` / `llvm_backend_surrogate.rs` are now frozen exact owners; docs/inventory closeout only until caller-proof says the temporary lane can disappear
  - `src/runner/modes/common_util/exec.rs` no longer carries dead `llvmlite_emit_object(...)`, so runner-side llvmlite residue shrank by one helper
  - `lang/src/shared/backend/backend_recipe_box.hako` dropped dead `prepare_compile_route(...)` / `prepare_compile_pure_first(...)` wrappers, so route-profile ownership is now tighter
  - `lang/src/shared/backend/backend_recipe_box.hako` no longer exposes `prepare_link_route(...)`; link `libs` normalization is now inlined in `LlvmBackendBox.link_exe(...)`
  - `lang/src/shared/backend/backend_recipe_box.hako` now inlines the route/profile owner constants and the `pure-first` / `harness` recipe labels directly, so the route-profile owner has no extra recipe indirection left
  - `lang/src/shared/backend/llvm_backend_box.hako` now forwards the caller `json_path` directly into `env.codegen.compile_json_path(json_path, "", recipe, compat)` after route-profile validation
  - `lang/src/runtime/kernel/string/search.hako` dropped dead `_starts_with_at(...)`, so the string-search owner is thinner without widening scope
  - `lang/src/runtime/kernel/string/search.hako` also folded `_find_index_core(...)` into `find_index(...)`, so the string-search control structure is a bit flatter
  - `lang/src/shared/host_bridge/codegen_bridge_box.hako` is now args-only; the 1-arg convenience wrappers were removed and `stage1_cli` / `LLVMEmitBox` moved to `*_args`
  - `lang/src/runtime/host/host_facade_box.hako` now calls `CodegenBridgeBox.*_args` directly, with its pass-through helper layer removed; the VM boxcall path now routes compile/link effects through `lang/src/vm/boxes/mir_vm_s0_codegen.hako`
  - `BackendRecipeBox.compile_route_profile(...)` now treats `acceptance_case` rows as grouped evidence buckets rather than per-case transport trivia
  - `BackendRecipeBox.compile_route_profile(...)` now owns the exact route-profile contract validation, so `LlvmBackendBox` can stay transport-focused when calling `env.codegen.*`
  - `BackendRecipeBox.require_non_empty_field(...)` is now the shared backend input guard, so `LlvmBackendBox` no longer carries its own duplicate non-empty validation helper
  - `CodegenBridgeBox` now owns the legacy optional-arg `env.codegen.*` normalization used by `HostFacadeBox`, so that caller shape lives in one shared bridge instead of being duplicated
  - `MirVmS0CodegenBox` now owns the VM-side compile/link apply helpers and uses `CodegenBridgeBox.*_args` internally, keeping `MirVmS0BoxcallExecBox` focused on file/string/misc routing
  - `LLVMEmitBox` now keeps its provider-stub slice thin by splitting input validation from provider dispatch; it remains a strict provider router and does not widen provider policy
  - `lang/src/runtime/kernel/string/search.hako` dropped the dead `_starts_with_at(...)` helper; all callers use `_starts_with_at_norm(...)` directly
  - legacy `phase2034/llvmemit_canary_vm.sh` and `phase2034/llvmemit_llvmlite_canary_vm.sh` were deleted; they are not part of the active smoke set
  - worker inventory: `src/host_providers/llvm_codegen/route.rs` cannot yet drop `requested_compile_recipe` / `requested_compat_replay` because legacy `env.codegen.emit_object` and `env.codegen.compile_json_path` callers still pass `None`; this lane is inventory-only until caller-side route/profile explicitization lands
  - `lang/src/runtime/host/host_facade_box.hako` and `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako` now delegate the legacy optional `env.codegen.emit_object` / `env.codegen.compile_json_path` caller shape to `CodegenBridgeBox`; behavior stays unchanged and the daily owner remains `LlvmBackendBox`
- do not mix this slice with:
  - kernel migration refactors
  - `boundary_driver.rs` compat keep reduction
  - `native_driver.rs` bootstrap keep reduction
  - C shim transport micro-splitting
- restart read order:
  1. this file
  2. `lang/src/shared/backend/README.md`
  3. `lang/src/shared/host_bridge/README.md`
  4. `docs/development/current/main/design/backend-recipe-route-profile-ssot.md`
  5. `lang/src/shared/backend/backend_recipe_box.hako`

## Focus Lock (2026-03-02)

- primary target: `kernel-mainline`（`.hako` kernel）を日常既定経路に固定。
- no-fallback: `NYASH_VM_USE_FALLBACK=0`（silent fallback 禁止 / fail-fast）。
- compiler lane は `phase-29bq` を monitor-only 運用（failure-driven reopen のみ）。

## Kernel Migration First (2026-03-20)

- current main goal:
  - kernel authority migration を collection owner cutover として先に終わらせてから raw substrate optimization に進む
  - plan SSOT: `docs/development/current/main/phases/phase-29cm/README.md`
  - `0rust` は Rust meaning owner zero の意味であり、Rust ベースの build/bootstrap route は常時保持する
  - operationally, `stage0` Rust bootstrap keep is allowed; target the `stage2+` selfhost mainline for `0rust`
  - current `.hako` authoring lane is no longer `string` widening; `string` is parked at stop line while collection owner cutover reopens
  - fixed order:
    1. `string`
       - `string.search` v0 は landed 済み。これ以上の widening は新しい exact blocker が出るまで pause
    2. `array`
       - collections ring1 を first owner に保つのではなく、`.hako` ring1 collection core owner へ昇格する
       - move `get/set/push/len/length/size`, bounds policy, normalization, fallback, visible error contract into `.hako`
       - Rust `array` plugin/helpers are demoted to raw storage/cache/load/store substrate verbs only
    3. `map`
       - `array` のあとで `.hako` ring1 collection core owner へ昇格する
       - move `get/set/has/len/length/size`, key normalization, visible error contract into `.hako`
       - Rust `map` plugin/helpers are demoted to hash/probe/rehash/layout substrate verbs only
    4. `runtime_data cleanup`
       - `RuntimeDataBox` は owner へ昇格させず、protocol / facade に固定する
       - array/map semantics を吸い込む god-box 化は禁止
    5. `numeric`
       - `MatI64.mul_naive` landed 済みの narrow pilot として parked
       - collection owner cutover が落ち着くまで新しい narrow op は増やさない
  - perf / asm optimization は raw substrate 境界を固定してから follow-up に回す

## Backend-Zero Next (queued)

- current hard lane after kernel stop line:
  - backend-zero を active lane にする
  - fixed order / buildability gate は `docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md` を正本にする
  - order:
    1. current owner cutover
    2. compat keep reduction
    3. bootstrap keep reduction
- rule:
  - buildability は gate contract であり authority ではない
  - kernel migration の再調整と同じ slice で backend-zero を進めない
  - Rust build/bootstrap route は常時保持する

## Exe Optimization Wave (secondary / parked behind backend-zero handoff) (2026-03-18)

- follow-up goal:
  - `kilo` / `micro kilo` の exe 最適化に移る
  - C / Python / Hako を stable baseline と micro ladder で比較し、確認できた hot leaf だけを詰める
  - `.hako` の `@hint(inline)` は advisory trial としてのみ使う。意味を変える workaround にはしない
  - C ABI / bridge / loader は transport-only に寄せ、policy は `.hako` に残す
  - surface / boundary protocol は `Everything is Box` のまま維持し、low-level string algorithm control structure は `.hako` / docs 側の owner として読む
  - raw byte scan / compare / copy / allocation / freeze leaf は当面 Rust/C substrate に残し、この wave では leaf substrate delete と混ぜない
  - perf AOT lane is `.hako -> ny-llvmc(boundary) -> C ABI`; `llvmlite` / `native` / harness keep lanes are not valid in this wave
  - `.hako` string kernel op set v0 の narrow pilot は `lang.runtime.kernel.string.search` で切り、`find_index` / `contains` を先に固定する
  - current narrow pilot landed:
    - `lang/src/runtime/kernel/string/search.hako` now owns `find_index` / `contains` / `starts_with` / `ends_with` / `split_once_index`
    - `lang/src/runtime/kernel/hako_module.toml` exports it as `string.search`
    - `apps/tests/string_kernel_search_min.hako` is the direct VM fixture
    - `apps/tests/string_kernel_starts_with_min.hako` is the direct VM fixture for the next narrow op
    - `apps/tests/string_kernel_ends_with_min.hako` is the direct VM fixture for the latest narrow op
    - `apps/tests/string_kernel_split_once_index_min.hako` is the direct VM fixture for the latest narrow op
    - `lang/src/runtime/kernel/numeric/matrix_i64.hako` now owns `MatI64.mul_naive` loop/body and `lang/src/runtime/numeric/mat_i64_box.hako` is the thin `new MatI64(rows, cols)` wrapper
  - next `.hako` kernel family order after string.search v0:
    1. `array` kernel family on the same runtime/kernel lane
       - defer -> kernel promotion is trigger-based, not calendar-based: keep `ArrayBox.length/len/size` in `lang/src/runtime/collections/array_core_box.hako` while it stays wrapper-only, and move to `lang/src/runtime/kernel/array/` only when a concrete policy difference appears (owner-local policy/normalization/birth handling, or a dedicated acceptance row / smoke that cannot stay as a thin ring1 wrapper)
    2. `numeric` kernel family after array stabilizes
       - first narrow pilot is `MatI64.mul_naive` in `lang/src/runtime/kernel/numeric/`
       - ring1 wrapper stays in `lang/src/runtime/numeric/` and only delegates to the kernel owner
    3. `map` stays in `lang/src/runtime/collections/` ring1 and is not part of this kernel lane
  - next narrow op order for this lane:
    1. `ArrayBox.length/len/size` observer path stays in collections ring1 first; do not create a new array kernel module yet
    2. further widening paused until a new exact blocker appears; if none appears, stop the lane and move to inventory or the next fixed order
  - landed array thin slice:
    - `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now returns the observer-only `ArrayBox.length/len/size` alias before `set/get/push` stateful prep, so the ring1 wrapper stays thin without opening `lang/src/runtime/kernel/array/`
  - quick array canary shape:
    - `tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh` now uses `print(a.length())` directly; `toString` is treated as a separate blocker and no longer hides array-length canary failures
  - landed array stateful thin slice:
    - `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now delegates `set/get/push` into owner-local helpers, so observer path and stateful write path are split without changing the ring1 defer boundary
  - landed array observer fast-path slice:
    - `lang/src/runtime/collections/array_core_box.hako::try_handle(...)` now keeps `ArrayBox.length/len/size` on a lazy observer-only fast path and delays stateful len/key plumbing until `set/get/push` is actually selected
  - landed array stateful helper split:
    - `lang/src/runtime/collections/array_state_core_box.hako` now owns `record_push_state(...)` / `record_set_state(...)` / `get_state_value(...)`, so `array_core_box.hako` is more router-only while remaining in collections ring1
  - landed array plugin helper split:
    - `crates/nyash_kernel/src/plugin/array.rs` now delegates handle-based get/set/has route helpers into `crates/nyash_kernel/src/plugin/array_route_helpers.rs`, so the substrate file is thinner while the ring1 defer boundary stays unchanged
  - landed array string-slot helper split:
    - `crates/nyash_kernel/src/plugin/array_route_helpers.rs` now delegates string-handle slot retargeting for `set_his` into `crates/nyash_kernel/src/plugin/array_string_slot.rs`, so the route helper is more route-only while the ring1 defer boundary stays unchanged
- owner scope lock for this wave:
  - touch-first owners:
    - `crates/nyash_kernel/src/exports/string.rs`
    - `crates/nyash_kernel/src/exports/string_view.rs`
    - `crates/nyash_kernel/src/plugin/string.rs`
    - `src/runtime/host_handles.rs`
    - `lang/c-abi/shims/hako_aot.c`
    - `lang/c-abi/shims/hako_aot_shared_impl.inc`
  - do-not-touch owners unless the route contract itself is broken:
    - `src/llvm_py/**`
    - `tools/llvmlite_harness.py`
    - `crates/nyash-llvm-compiler/src/harness_driver.rs`
    - explicit keep-lane selectors (`NYASH_LLVM_BACKEND=llvmlite|native`, `NYASH_LLVM_USE_HARNESS=1`, `HAKO_LLVM_EMIT_PROVIDER=llvmlite`)
  - investigation rule:
    - start from the asm top symbol owner, not from keyword grep hits
    - if the top symbol lives under kernel/runtime/C boundary, do not reopen `llvm_py` in this wave
- canonical entry points:
  - `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_hk auto 5 5 11`
  - `bash tools/perf/run_kilo_micro_machine_ladder.sh 1 15`
  - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 15`
  - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 15`
  - `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 15`
- decision rule:
  - `ratio_c_aot >= 0.95` かつ `aot_status=ok` なら、その lane は monitor-only に落とす
  - それ未満なら、micro ladder で次の exact hot leaf を選んで続きを詰める
- fresh-build rule:
  - `AUTO` skip-build の baseline は診断専用
  - first baseline は `PERF_AOT_SKIP_BUILD=0` で取り直して、release binary の stale mix を避ける
- latest verified baseline (2026-03-18):
  - `kilo_kernel_small_hk` -> `c_ms=79`, `py_ms=111`, `ny_vm_ms=989`, `ny_aot_ms=804`, `ratio_c_aot=0.10`, `aot_status=ok`
  - the benchmark is still bridge/helper-density bound; the next move is exact leaf trimming, not route rewrites
- latest verified micro ladder (2026-03-18):
  - `kilo_micro_substring_concat` is the thickest lane
  - `kilo_micro_array_getset` is next
  - `kilo_micro_indexof_line` is the least bad of the three
  - after caching hot bridge trace env probes, the ladder is still ordered the same, but `indexof_line` and `substring_concat` moved down a little in AOT cycles
  - current round (2026-03-18): `indexof_line ratio_cycles=0.12 ny_aot_cycles=61408570`, `substring_concat ratio_cycles=0.08 ny_aot_cycles=5857310`, `array_getset ratio_cycles=0.01 ny_aot_cycles=228000964`
  - current exact leaf slice for `substring_concat`: `crates/nyash_kernel/src/exports/string_view.rs` now owns `borrowed_substring_plan_from_handle(...)`, so `crates/nyash_kernel/src/exports/string.rs::substring_hii` stays a thin dispatch/match wrapper while the hot path remains on direct `with_handle(...)` instead of cache-backed span lookup
  - accepted structure-first slice: `crates/nyash_kernel/src/exports/string.rs::concat3_hhh` is now split into `concat3_plan_from_parts(...)` / `concat3_plan_from_fast_str(...)` / `concat3_plan_from_spans(...)` plus `freeze_concat3_plan(...)`, so route selection and birth are separated file-locally without reopening substrate semantics
  - current runtime follow-up slice for `substring_concat`: `src/runtime/host_handles.rs::Registry::alloc` now reads `policy_mode` before the write lock and keeps invariant panics in cold helpers, so the success path stays straight-line
  - current exact leaf slice for `array_getset`: the read seam (`crates/nyash_kernel/src/plugin/array_slot_load.rs::array_slot_load_encoded_i64`) is landed and kept
  - raw substrate perf is parked again until the collection boundary deepens below the transitional method-shaped Rust exports still used by `.hako` owners
  - next non-perf task is to deepen the hidden raw-named residue below:
    - `nyash.array.slot_append_hh`
    - `nyash.array.slot_store_hii`
    - `nyash.map.slot_* / probe_*`
  - landed boundary-deepen step:
    - daily `.hako` array observer route now uses `nyash.array.slot_len_h`
    - daily `.hako` array append route now uses `nyash.array.slot_append_hh`
    - daily `.hako` map observer route now uses `nyash.map.entry_count_h`
  - `crates/nyash_kernel/src/plugin/array_index_helpers.rs` / `array_route_helpers.rs` are now thin wrappers, so they are no longer the primary P1 target
  - fresh `kilo_micro_array_getset` recheck after the read-seam keep is `ny_aot_ms=43` (`ratio_cycles=0.01`)
  - rejected probes (reverted immediately): dedicated i64 write helper regressed to `47 ms`, and `try_set_index_i64_integer` cold-split regressed to `48 ms`
  - fresh `bench_micro_aot_asm.sh kilo_micro_array_getset` now shows `array_slot_store_i64` closure plus `LocalKey::with` as the dominant pair, but that probe stays parked until the collection boundary is deeper
  - current contract-change slice: `crates/nyash_kernel/src/exports/string_view.rs` now allows `<= 8 bytes` short slices to eager-materialize instead of always creating `StringViewBox`; this reduces view churn and improves stable whole-program baseline, even though the isolated micro leaf regressed slightly
  - rejected experiment (not kept): splitting the policy to `root StringBox <= 16 bytes` / `nested StringViewBox <= 8 bytes` improved isolated `substring_concat` micro to `262468757 cycles / 69 ms`, but stable `kilo_kernel_small_hk` regressed to `819 ms`, so the wave stays on the flat `<= 8 bytes` policy while whole-program stable remains the primary metric
  - rejected experiment (reverted immediately): `crates/nyash_kernel/src/exports/string.rs::string_len_from_handle(...)` tried explicit `StringBox` / `StringViewBox` downcast fast paths; isolated `substring_concat` micro landed at `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed hard to `1066 ms` median (`min=786`, `max=1841`), so observer-only fast path is not a keep candidate
  - rejected structure-first experiment (reverted immediately): moving `StringViewBox` birth out of `borrowed_substring_plan_from_handle(...)` into `substring_hii` via `BorrowedSubstringPlan::{OwnedSubstring,ViewRecipe}` kept the planner thinner and matched the future `freeze` direction, but it was only a birth-site shuffle, not a real transient layer; isolated `substring_concat` landed at `267397179 cycles / 72 ms`, while stable `kilo_kernel_small_hk` regressed to `901 ms` median (`min=794`, `max=1146`), so this cut is do-not-repeat until a larger `TStr`/freeze boundary exists
  - fresh recheck after the current kept slices: `substring_concat ny_aot_cycles=266244455 ny_aot_ms=72` on `bench_micro_c_vs_aot_stat.sh ... 1 9`; stable `kilo_kernel_small_hk` is `798 ms` median (`min=791`, `max=1607`), and the `concat3_hhh` plan/freeze split stays accepted because the median improved below the previous `804 ms` line despite one noisy outlier round
  - stop-line for this exact wave: `BoxBase::new` is identity-bound and is not a safe optimization target; the next cut must reduce `StringViewBox::new` call count or another upstream owner, not reuse box IDs
- likely remaining hotspot family from the last inventory:
  - `TLS / LocalKey::with`
  - `array_get_hi`
  - `find_substr_byte_index`
- read-first for this wave:
  - `docs/development/current/main/design/perf-optimization-method-ssot.md`
  - `docs/development/current/main/design/optimization-tag-flow-ssot.md`
  - `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
  - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`
  - `docs/development/current/main/design/string-transient-lifecycle-ssot.md`
  - `docs/development/current/main/design/rep-mir-string-lowering-ssot.md`
  - `lang/src/runtime/kernel/string/README.md`
  - `docs/development/current/main/design/box-identity-view-allocation-design-note.md`
  - `docs/development/current/main/design/boxbase-new-external-consultation-question.md`
  - `docs/development/current/main/design/transient-string-chain-external-consultation-question.md`
  - `docs/development/current/main/design/kernel-authority-cutover-external-consultation-question.md`
  - `docs/development/current/main/investigations/phase21_5-kilo-hotspot-triage-2026-02-23.md`
  - `docs/development/current/main/investigations/substring-concat-observer-fast-path-and-upstream-cut-2026-03-18.md`

## Next Wave Board (queued)

- main goal:
  - `substring -> concat3 -> length` chain を more transient / span-first に寄せ、rejected threshold split を reopen せずに box birth を減らす
  - `observable` ではなく `substrate-visible / retained` を birth ルールにし、loop-carried `text = out.substring(...)` を first escape boundary として扱う
  - code を `authority / transient / birth boundary / substrate` の 4 層で読める形へ寄せる
  - surface は `Everything is Box` を維持しつつ、low-level string algorithm control structure を `.hako` owner に寄せ、raw byte/memory/freeze leaf は substrate に残す
- design SSOT:
  - `docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md`
  - `docs/development/current/main/design/string-transient-lifecycle-ssot.md`
  - `docs/development/current/main/design/rep-mir-string-lowering-ssot.md`
  - `docs/development/current/main/design/rep-mir-string-birth-map-inventory.md`
  - `lang/src/runtime/kernel/string/README.md`
  - `lang/src/runtime/kernel/string/search.hako`
- Rust growth lock:
  - `RepMIR` / `freeze-thaw` pilot is allowed only as a temporary AOT backend-local consumer
  - owner of `RepKind`, birth rule, and escape rule stays in docs / `.hako authority`, not in private Rust-only optimizer logic
  - do not widen pilot scope beyond `kilo_micro_substring_concat` until the first narrow lane proves out
  - do not add a new runtime layer, `NyashBox` variant, or host-handle token type for this pilot
  - best placement is `ny-llvmc(boundary)`-side shadow lowering, not `crates/nyash_kernel/src/exports/string.rs` as a new permanent owner
  - canonical contract naming:
    - Shadow RepMIR op names stay `thaw.str`, `str.slice`, `str.concat3`, `str.len`, `str.find_byte_from`, `str.eq_at`, `freeze.str`
    - `.hako` kernel-side intrinsic spellings use `__str.*` and map 1:1 onto those ops
- owner scope lock:
  - touch-first owners:
    - `crates/nyash_kernel/src/exports/string.rs`
    - `crates/nyash_kernel/src/exports/string_view.rs`
  - read-only owners:
    - `crates/nyash_kernel/src/exports/string_span_cache.rs`
  - do-not-touch owners unless the route contract itself breaks:
    - `src/runtime/host_handles.rs`
    - `src/llvm_py/**`
    - `tools/llvmlite_harness.py`
    - `lang/c-abi/shims/hako_aot.c`
    - `lang/c-abi/shims/hako_aot_shared_impl.inc`
- design-first tasks:
  1. transient chain / escape boundary / 4-layer reading を SSOT で固定する
  2. current code の `plan` と `birth` の混線点を `substring_hii` / `concat3_hhh` / `string_handle_from_owned` / `borrowed_substring_plan_from_handle(...)` で棚卸しする
  3. current flat `<= 8 bytes` policyを変えないまま、future `freeze` boundary へ寄せる structure-first slice だけ試す
     - rejected and do-not-repeat first: `string_len_from_handle` explicit downcast observer fast path
     - rejected and do-not-repeat first: planner-only `OwnedSubstring/ViewRecipe` split that merely moves `StringViewBox` birth from planner to `substring_hii` without introducing a real transient carrier
     - accepted first: file-local `concat3_hhh` `plan -> freeze` split; continue only if the next slice reduces actual birth density rather than just shuffling call sites
  4. `Shadow RepMIR` docs-first pilot を `kilo_micro_substring_concat` 限定で定義し、Rust を temporary backend-local lane に固定する
  5. `substring_hii` / `concat3_hhh` / `string_len_from_handle` / `string_handle_from_owned` / `borrowed_substring_plan_from_handle(...)` の birth map を 1 枚に固定する
  6. `.hako authority / Rust substrate` の string owner map を維持したまま shadow-owner wave へ備える
  7. `substring_concat` pilot と別に、search/control kernel wave は `.hako` string kernel op set v0 として narrow に切り出し、黙って widen しない
     - current public surface is `find_index` / `contains` / `starts_with` / `ends_with` / `split_once_index`
     - further widening paused until a new exact blocker appears; if none appears, stop the lane and move to inventory or the next fixed order
- acceptance:
  1. `cargo test -q -p nyash_kernel substring_hii -- --nocapture`
  2. `cargo test -q -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
  3. `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 9`
  4. `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_hk auto 5 5 11`
  5. stable median を `804 ms` 以下に維持し、micro-only improvement は keep 根拠にしない

## Quick Task Board (2026-03-17)

- note:
  - this block is legacy backend-zero / llvmlite closeout history; the current active lane is collection owner cutover above
- legacy closeout snapshot:
  - `backend-zero` を final shape `.hako -> LlvmBackendBox -> hako_aot -> backend helper` へ寄せる
  - `src/host_providers/llvm_codegen.rs`, `crates/nyash-llvm-compiler/src/main.rs`, `crates/nyash-llvm-compiler/src/native_driver.rs` は途中の Rust glue / keep lane であり、final owner ではない
  - `lang/c-abi/shims/hako_llvmc_ffi.c` は急いで delete せず、まず `transport-only` の tiny C substrate に縮める
  - `.hako` の next exact slice は `BackendRecipeBox` の route profile SSOT 化で、policy owner / transport owner / compile recipe / compat replay を 1 枚の profile で明示すること
  - landed recipe classification row:
  - `BackendRecipeBox.compile_route_profile(...)` now also names `acceptance_policy=boundary-pure-seed-matrix-v1`
  - `ret_const_min_v1` now also has an explicit `.hako` evidence row via `acceptance_case=ret-const-v1`
  - `hello_simple_llvm_native_probe_v1` now also has an explicit `.hako` evidence row via `acceptance_case=hello-simple-llvm-native-probe-v1`
  - `RuntimeDataBox.get(ArrayBox missing index)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-array-get-missing-v1`
  - `RuntimeDataBox.length(StringBox)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-string-length-ascii-v1`
  - `RuntimeDataBox.length(ArrayBox)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-array-length-v1`
  - `RuntimeDataBox.push(ArrayBox)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-array-push-v1`
  - `RuntimeDataBox.length(MapBox)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-map-size-v1`
  - `RuntimeDataBox.has(ArrayBox missing index)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-array-has-missing-v1`
  - `RuntimeDataBox.has(MapBox missing key)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-map-has-missing-v1`
  - `RuntimeDataBox.get(MapBox missing key)` now also has an explicit `.hako` evidence row via `acceptance_case=runtime-data-map-get-missing-v1`
  - `StringBox.indexOf/1` now also has an explicit `.hako` evidence row via `acceptance_case=string-indexof-ascii-v1`
  - `StringBox.length/size` now also has an explicit `.hako` evidence row via `acceptance_case=string-length-ascii-v1`
    - this is the first visible `.hako`-owned label for why daily route stays `pure-first + harness`, while transport behavior remains unchanged
  - rust-layer inventory for the next cleanup wave:
    - `src/host_providers/llvm_codegen.rs` has already split its route-selection helpers into `src/host_providers/llvm_codegen/route.rs` and now also delegates MIR normalization / transport helpers into `src/host_providers/llvm_codegen/normalize.rs` plus `src/host_providers/llvm_codegen/transport.rs`, so the parent file is now closer to orchestration glue than policy owner
    - `crates/nyash-llvm-compiler/src/main.rs` is CLI routing glue and now delegates input shaping into `crates/nyash-llvm-compiler/src/compile_input.rs` plus emit/link driver dispatch into `crates/nyash-llvm-compiler/src/driver_dispatch.rs`; `driver_dispatch.rs` now further delegates Python-harness duties into `crates/nyash-llvm-compiler/src/harness_driver.rs` and link/finalize duties into `crates/nyash-llvm-compiler/src/link_driver.rs`, so Rust thin-up is now at its stop line and the next move is exe optimization
    - `crates/nyash-llvm-compiler/src/boundary_driver.rs` is transport-only keep and should stay out of the daily-owner cutover
    - `crates/nyash-llvm-compiler/src/native_driver.rs` is bootstrap/canary keep only
    - `src/runner/modes/llvm/object_emitter.rs` is already thin and is not the first deletion target
  - remaining backend-zero tasks, in order:
    1. `phase-29cl` caller-cutover closeout: `by-name` は daily mainline から既に外れており、compat/archive residue だけを維持する
    2. LLVM daily exe/object route から `llvmlite` と `native_driver` の両方を外す
    3. `BackendRecipeBox` の recipe/profile evidence rows は、新しい exact fixture が出るまで増やさない
    - `BackendRecipeBox.acceptance_case_for(...)` is now bucketed by string/runtime_data/loop/seed and keeps `boundary-pure-seed-matrix-v1` as the catch-all fallback
    4. `hako_llvmc_ffi.c` は export / marshal / fallback transport only に固定し、policy を戻さない
  - `BYN-min4` hook/registry bridge and its Rust register entrypoints are now compat-only and crate-private; remaining by-name work is kernel hard retire readiness after caller shrink
  - `by_name.rs` FileBox named-method compat tail now lives in `compat_invoke_core.rs`; remaining kernel caller shrink is `module_string_dispatch.rs` / compiled-stage1 surrogate residue before hard retire readiness
  - `src/llvm_py/instructions/mir_call_legacy.py` now forwards receiver literals into the shared direct-or-plugin tail, so the legacy BuildBox module-string path can resolve direct lowered methods before `nyash.plugin.invoke_by_name_i64`
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs` tests are now direct-dispatch only; by-name compat proof lives elsewhere and is no longer owned there
  - `src/backend/mir_interpreter/handlers/boxes_file.rs` and `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako` are the current FileBox direct-contract owners to inspect before any `by_name` delete
  - current clean stop-line:
    - `.hako` policy owner is `BackendRecipeBox`
    - `.hako` caller stop-line is `LlvmBackendBox -> env.codegen.*`
    - Rust is payload decode / symbol selection / boundary glue only
    - C is export / marshal / loader / process / compat transport only
    - do not reopen C micro-thinning or broad pure-seed widening unless a new exact blocker requires it
- already stopped:
  - bootstrap closure wave は fixed-point compare まで完了
  - `stage7 launcher` / `stage9 launcher` と fresh `stage1-cli` rebuild は byte-identical
  - 以後は fresh semantic mismatch が出ない限り `stageN` を増やさない
  - active order:
    1. `phase-29cl` caller-cutover closeout: `by-name` は daily mainline から既に外れており、compat/archive residue だけを維持する
    2. LLVM daily exe/object route から `llvmlite` と `native_driver` の両方を外す
    3. backend-zero は `.hako = policy/recipe owner`, `C = export/transport owner` に固定し、C を急いで delete しない
    4. Rust thin-up はここで止めて、次は exe optimization に移る
    4. `llvmlite` / `native_driver` は compat/canary keep に固定したまま、boundary-owned default route を thin floor まで固める
    5. kernel-side `by_name` は retire 済みのまま維持し、新しい live caller が現れた時だけ reopen を再判定する
- freeze unless blocker:
  - `phase-29cj` micro-thinning
  - bridge/program-json/stub-emit cleanup
  - string coercion cleanup
  - compiled-stage1 surrogate shrink
- read-first for this lane:
  - `docs/development/current/main/phases/phase-29cl/README.md`
  - `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
  - `docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md`
  - `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`
  - `docs/development/current/main/design/backend-recipe-route-profile-ssot.md`
  - `docs/development/current/main/design/frontend-owner-proof-index.md`
  - `lang/src/shared/backend/README.md`

## Full Rust 0 Pointer (2026-03-14)

- top-level future tracking SSOT:
  - `docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md`
  - `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
  - `docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-inventory-ssot.md`
  - `docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md`
- split:
  - `runtime-zero`: accepted pointer / inventory-ready
    - collection owner SSOT: `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`
    - ring lock: `array` / `map` are `ring1`, not `ring0`
    - current truth: runtime/provider lane is still wired by Rust `src/providers/ring1/{array,map}/mod.rs`, while AOT/LLVM collection paths still rely on Rust `crates/nyash_kernel/src/{exports/birth.rs,plugin/array.rs,plugin/map.rs,plugin/runtime_data.rs}` and `.hako` `lang/src/runtime/collections/**` / `lang/src/vm/boxes/abi_adapter_registry.hako` are thin wrapper/adapter owners
    - latest visible-owner slices:
      - `lang/src/runtime/collections/map_core_box.hako` now owns adapter-on `MapBox.{set,get,has,size/len/length}` orchestration plus size/state helpers consumed by `lang/src/vm/boxes/mir_call_v1_handler.hako`
      - `lang/src/runtime/collections/array_core_box.hako` now owns adapter-on `ArrayBox.{set,get,push,len/length/size}` orchestration plus len/state helpers consumed by the same handler
      - `lang/src/runtime/collections/runtime_data_core_box.hako` now owns narrow `RuntimeDataBox.{get,set,has,push}` method dispatch plus the same extern routes consumed by `lang/src/vm/boxes/mir_call_v1_handler.hako`
      - `lang/src/runtime/collections/string_core_box.hako` now owns adapter-on `StringBox.length/len/size` orchestration plus the `nyash.string.len_h` thin extern route consumed by the same handler
      - `src/providers/ring1/array/mod.rs` now keeps `Ring1ArrayService` type-gate / index boxing behind owner-local helpers and locks invalid-type contract with unit tests, so runtime/provider lane is slightly thinner without changing semantics
      - `src/providers/ring1/map/mod.rs` now keeps `Ring1MapService` type-gate / key boxing / size-bool extraction behind owner-local helpers and locks invalid-type contract with unit tests
    - latest proof lock:
      - `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` is the source-contract lock for `StringCoreBox` / `ArrayCoreBox` / `MapCoreBox` / `RuntimeDataCoreBox` wiring under `lang/src/vm/boxes/mir_call_v1_handler.hako`
      - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` is the current standalone AOT/runtime-data e2e smoke for `apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json`
      - note: `tools/smokes/v2/lib/test_runner.sh::verify_v1_inline_file()` still routes `HAKO_VERIFY_PRIMARY=hakovm` through `src/main.rs` `hv1_inline::run_json_v1_inline(...)`, so phase2170 hakovm-primary canaries remain Rust hv1_inline proofs, not `.hako` `MirCallV1HandlerBox` owner proofs
    - exact next front:
      - keep `tools/smokes/v2/profiles/integration/apps/archive/phase29x_runtime_data_dispatch_contract_vm.sh` as deferred lower-level cargo-test contract keep
      - runtime/provider lane is monitor-only after helper-thinning in `src/providers/ring1/{array,map}/mod.rs`
      - next `.hako ring1` front is only worth reopening when a new narrow collection/runtime seam appears with fixture+gate value; current collection adapter-on orchestration slices are landed for `ArrayBox`, `MapBox`, `RuntimeDataBox`, and `StringBox` size aliases, so this lane is now near thin floor
      - `ArrayCoreBox` / `MapCoreBox` size-state fallback tails are compat cleanup only, not blocker work
    - target lock: move mainline collection ownership toward `.hako ring1` collection/runtime layer first, then shrink Rust births/plugins/builtin residue to compat/archive keep
  - `kernel-authority-zero`: queued pointer
    - SSOT: `docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md`
    - meaning/policy owner を `.hako` 側へ寄せる波であり、Rust substrate delete と同じ task にしない
    - historical rule at this snapshot: active exe optimization wave と混ぜない
    - historical start trigger at this snapshot: current perf / backend-zero stop-line が揃ってからだけ reopen する
  - `backend-zero`: accepted pointer / `phase-29ck` queued
    - boundary SSOT: `docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md`
    - fixed order / buildability gate: `docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md`
    - design SSOT: `docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md`
    - phase SSOT: `docs/development/current/main/phases/phase-29ck/README.md`
    - post-cutover by-name retirement SSOT: `docs/development/current/main/phases/phase-29cl/README.md`
    - final shape lock: `.hako -> thin backend C ABI/plugin boundary -> object/exe`
    - `crates/nyash-llvm-compiler/src/native_driver.rs` は bootstrap seam only
    - design lock (2026-03-18):
      - `.hako` is the owner for `pure-first seed selection`, `route selection`, `compile recipe`, `unsupported-shape classification`, `compat replay policy`, and backend diagnostics/fail-fast policy
      - recipe-aware daily callers should prefer explicit `pure-first` transport exports; the generic C export remains a forwarder / historical compat surface
      - `lang/c-abi/shims/hako_llvmc_ffi.c` stays as a tiny C substrate for `extern "C"` export, `char** err_out` ownership, allocator boundary, `dlopen/dlsym`, `system()`, and path/env/process glue
      - do not optimize for `0C`; optimize for `0rust + policy-in-.hako + transport-only C`
      - `hako_llvmc_ffi.c` should lose meaning/policy before it loses existence
    - landed slice:
      - `lang/src/shared/backend/llvm_backend_box.hako` の first implementation
      - `compile_obj(json_path)` / `link_exe(obj_path, out_path, libs)` を thin caller facade に固定し、owner は `CodegenBridgeBox` に寄せた
      - acceptance は `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh` と `tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh` で lock 済み
      - `lang/c-abi/include/hako_aot.h` を AOT compile/link 宣言の canonical header に固定し、`hako_hostbridge.h` は thin shim 化した
      - `lang/c-abi/shims/hako_aot_shared_impl.inc` を shared source truth にして、`hako_aot.c` / `hako_kernel.c` の AOT compile/link 実装を 1 箇所へ寄せた
      - latest BE0-min6 follow-up: `lang/c-abi/shims/hako_diag_mem_shared_impl.inc` now owns shared TLS diagnostics + libc memory for `hako_aot.c` / `hako_kernel.c`, so `hako_aot_shared_impl.inc` stays compile/link-only
      - latest BE0-min6 command/log cleanup: `lang/c-abi/include/hako_aot.h` now names the path-owner contract explicitly (`mir_json_path` / `obj_path` / `exe_path`), and `lang/c-abi/shims/hako_aot_shared_impl.inc` now keeps compile/link command + log handling behind owner-local helpers
      - latest BE0-min6 resolution cleanup: `lang/c-abi/shims/hako_aot_shared_impl.inc` now keeps FFI library selection and runtime archive path resolution behind owner-local helpers, instead of repeating env/default path probes inline
      - latest BE0-min6 execute-fail cleanup: `lang/c-abi/shims/hako_aot_shared_impl.inc` now keeps compile/link `system(cmd)` failure projection, log cleanup, and compile success-finalize behind shared helpers instead of repeating them inline
      - latest BE0-min6 linker-finalize cleanup: `lang/c-abi/shims/hako_aot_shared_impl.inc` now keeps shim flag resolution, OS libs, PIE avoid, and appended linker-option keeps behind owner-local helpers instead of mixing them inline in `hako_aot_link_obj(...)`
      - latest runner/host-provider demotion: `src/host_providers/llvm_codegen.rs` now splits `C-API keep` / explicit `{llvmlite|ny-llvmc}` keep / boundary-first default through owner-local helpers, so the host-provider default object path now tries the direct C ABI boundary first and only the explicit keep lane may route through the `ny-llvmc` wrapper while `link_object_capi(...)` now passes linker keeps straight through to `hako_aot` instead of re-owning runtime archive / env ldflags synthesis in Rust
      - latest B1 arg-plumbing: `LlvmBackendBox.link_exe(obj_path, out_path, libs)` now forwards non-empty `libs` as the third `env.codegen.link_object` arg, and vm-hako / regular VM link handlers accept `[obj_path, exe_out?, extra_ldflags?]` while empty `libs` still falls back to `HAKO_AOT_LDFLAGS` under the C boundary
      - landed B1a/B1b: `CodegenBridgeBox` is documented as temporary bridge owner only, and `lang/src/runner/launcher.hako` `build exe` stop-point was first moved off direct `CodegenBridgeBox`
      - landed launcher Program(JSON)->MIR fix: `src/runner/pipe_io.rs` `--program-json-to-mir` now uses `src/host_providers/mir_builder.rs::program_json_to_mir_json_with_user_box_decls(...)`, so launcher MIR keeps root `user_box_decls` and the old `Unknown Box type: HakoCli` blocker is retired
      - landed launcher-exe backend boundary proof: compiled-stage1 module-string dispatch now owns temporary `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` surrogate handling in `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
      - launcher-exe `build exe -o ... apps/tests/hello_simple_llvm.hako` is now green under `NYASH_LLVM_USE_CAPI=1 HAKO_V1_EXTERN_PROVIDER_C_ABI=1`; the old `LlvmBackendBox.compile_obj failed` blocker is retired
      - landed B1c/B1d contract lock: `LlvmBackendBox.compile_obj(json_path)` is locked to the path-based thin backend contract, backend MIR normalization is owned by `src/host_providers/llvm_codegen.rs::normalize_mir_json_for_backend(...)`, compiled-stage1 `llvm_backend_surrogate.rs` shares the same path-based contract through `mir_json_file_to_object(...)`, and env truth is pinned to `NYASH_NY_LLVM_COMPILER` while `NYASH_LLVM_COMPILER` remains `tools/build_llvm.sh` selector only
      - landed B1e direct extern lowering: shared compile/link helpers in `lang/src/runtime/host/host_facade_box.hako` and `lang/src/vm/boxes/mir_vm_s0_boxcall_exec.hako` now lower directly to canonical `env.codegen.*` extern calls; daily compile/link no longer depends on `hostbridge.extern_invoke(...)`
      - landed `phase-29cl / BYN-min2`: `lang/src/runner/launcher.hako` `build exe` now calls `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)` directly, so the visible launcher source route no longer imports `selfhost.shared.backend.llvm_backend`; `llvm_backend_surrogate.rs` is now temporary compiled-stage1 residue only
      - landed B3a harness/entry demotion: `tools/llvmlite_harness.py` and `src/llvm_py/llvm_builder.py` now keep repo-root bootstrap, CLI parse, MIR file load, and output-file write behind owner-local helpers; `tools/llvmlite_harness.py` now calls `llvm_builder.build_object_from_input_file(...)` directly instead of re-entering the builder CLI through `runpy` + `sys.argv`; `NyashLLVMBuilder` / lowering/support remain untouched
      - landed B3b ingest/context first slice: `src/llvm_py/mir_reader.py` now owns normalized `BuilderInput` ingest, `src/llvm_py/build_opts.py` now owns `BuildOptions` env/codegen context, and `src/llvm_py/build_ctx.py` now owns lowering-side aggregated context via `build_ctx_from_owner(...)`; `src/llvm_py/llvm_builder.py` / `src/llvm_py/builders/instruction_lower.py` consume those seams without re-owning them inline
      - landed B3c opcode first slice: `src/llvm_py/instructions/by_name_method.py` once owned generic `nyash.plugin.invoke_by_name_i64` method fallback, and that wiring later moved to `src/llvm_py/instructions/plugin_invoke_lowering.py` while `boxcall.py` / `mir_call/method_call.py` / `mir_call_legacy.py` kept consuming the shared helper
      - landed B3c collection-route slice: `src/llvm_py/instructions/boxcall_runtime_data.py` now owns collection/runtime-data style `size/get/push/set/has` lowering, and `src/llvm_py/instructions/boxcall.py` now consumes that helper instead of carrying the route table inline
      - landed B3c collection-method slice: `src/llvm_py/instructions/mir_call/collection_method_call.py` now owns shared `get/push/set/has` route order for `mir_call/method_call.py` and `mir_call_legacy.py`
      - landed B3c method-tail slice: `src/llvm_py/instructions/mir_call/method_fallback_tail.py` now owns the final `direct known-box -> by-name plugin` route order for `mir_call/method_call.py` and `mir_call_legacy.py`
      - landed `phase-29cl / BYN-min4b`: `src/llvm_py/instructions/direct_box_method.py` now resolves the compiled-stage1 module-string receivers `lang.compiler.build.build_box` -> `BuildBox` and `lang.mir.builder.MirBuilderBox` -> `MirBuilderBox`, while `src/llvm_py/instructions/boxcall.py` and `src/llvm_py/instructions/mir_call/method_call.py` now pass receiver literals into that direct-call resolver so the daily `BuildBox.emit_program_json_v0(...)` / `MirBuilderBox.emit_from_program_json_v0(...)` pair no longer needs `nyash.plugin.invoke_by_name_i64` when direct lowered functions exist
      - landed `phase-29cl / BYN-min4c`: the same alias resolver now also covers `lang.compiler.entry.using_resolver(_box)` -> `Stage1UsingResolverBox`, and direct-first lowering now also reaches `Stage1UsingResolverBox.resolve_for_source(...)` plus `MirBuilderBox.emit_from_source_v0(...)` before generic plugin fallback when receiver literals are known
      - landed `phase-29cl / BYN-min4d`: the same alias resolver now also covers `selfhost.shared.backend.llvm_backend` -> `LlvmBackendBox`, so compiled-stage1 backend helper routes can prefer direct `LlvmBackendBox.compile_obj(...)` / `LlvmBackendBox.link_exe(...)` before generic plugin fallback when receiver literals are known
      - landed `phase-29cl / BYN-min4e`: `src/llvm_py/instructions/boxcall.py` no longer keeps a separate manual plugin tail; it now uses `src/llvm_py/instructions/mir_call/method_fallback_tail.py` as the shared direct-or-plugin owner, while preserving the legacy BoxCall `argc=min(len(args), 2)` compat contract through the shared helper
      - landed `phase-29cl / BYN-min4f`: the same direct-call alias resolver now also covers `lang.compiler.entry.func_scanner` -> `FuncScannerBox`, `lang.compiler.entry.stageb.stageb_json_builder_box` -> `StageBJsonBuilderBox`, `selfhost.shared.common.box_type_inspector` -> `BoxTypeInspectorBox`, and `selfhost.shared.common.string_helpers` -> `StringHelpers`, so compiled-stage1 helper routes such as `find_matching_brace`, `build_defs_json`, `kind`, and `int_to_str` can also prefer direct lowered functions before generic plugin fallback when receiver literals are known
      - landed B3c string/console-method slice: `src/llvm_py/instructions/mir_call/string_console_method_call.py` now owns shared `substring/indexOf/lastIndexOf/log` route order for `mir_call/method_call.py` and `mir_call_legacy.py`, while `length/size` specialization intentionally remains owner-local to `method_call.py`
      - landed B3d first slice: `src/llvm_py/build_ctx.py` now owns `current_vmap` / `lower_ctx`, and `src/llvm_py/builders/instruction_lower.py` consumes those context seams instead of reading `_current_vmap` / `ctx` off the builder owner inline
      - landed B3d resolver/type-facts slice: `src/llvm_py/type_facts.py` now owns shared `StringBox` / `ArrayBox` fact helpers and `src/llvm_py/resolver.py` now consumes them through owner-local `value_types` accessors, with proof pinned by `test_resolver_type_tags.py` and `test_type_facts.py`
      - landed B3d phi-manager slice: `src/llvm_py/phi_manager.py` now owns cross-block safety helpers for global-safe / PHI-owner / single-def dominance checks, and `filter_vmap_preserve_phis(...)` now reads as pure filter + predeclared merge with proof pinned by `test_phi_manager_snapshot_filter.py`
      - landed B3d mir-analysis slice: `src/llvm_py/mir_analysis.py` now keeps const-string scan and call-arity record behind owner-local helpers, so `scan_call_arities(...)` reads as function-level orchestration with proof pinned by `test_mir_analysis.py`
      - landed B3d phi-wiring-analysis slice: `src/llvm_py/phi_wiring/analysis.py` now keeps stringish seed classification and fixpoint propagation behind owner-local helpers, so `collect_produced_stringish(...)` reads as orchestration only with proof pinned by `test_phi_wiring.py`
      - landed B3d phi-wiring-tagging slice: `src/llvm_py/phi_wiring/tagging.py` now keeps PHI incoming sync, trivial-alias registration, placeholder registration, and tag propagation behind owner-local helpers, so `setup_phi_placeholders(...)` reads as block-level orchestration with proof pinned by `test_phi_tagging.py`
      - landed B3d phi-wiring-finalize slice: `src/llvm_py/phi_wiring/wiring.py` now keeps post-wire string/array/origin propagation behind owner-local helpers, and `src/llvm_py/phi_wiring/fact_propagation.py` now accepts both raw `(value, block)` and normalized `(block, value)` incoming shapes for ArrayBox carry, so `finalize_phis(...)` no longer mixes incoming wiring and resolver fact propagation inline, with proof pinned by `test_phi_wiring_finalize.py` and `test_phi_fact_propagation.py`
      - landed B3d phi-wiring-selection slice: `src/llvm_py/phi_wiring/wiring.py` now keeps snapshot candidate reuse, predecessor dedupe, self-carry normalization, incoming resolve/coercion, and per-pred selection behind owner-local helpers, so `wire_incomings(...)` now reads as `acquire phi -> match pred -> resolve/select -> add incoming` with proof pinned by `test_phi_wiring_selection.py`
      - landed B3d values-dominance slice: `src/llvm_py/utils/values.py` now keeps block-id/name extraction, same-block PHI detection, local def lookup, single-def dominance, PHI-owner dominance, and global-reuse checks behind file-local helpers, so `resolve_i64_strict(...)` reads as local-hit/global-hit/fallback orchestration with proof pinned by `test_resolve_i64_strict_scope.py`
      - landed B3d function-lower-prepass slice: `src/llvm_py/builders/function_lower.py` now keeps predecessor dedupe, block defs/uses collection, and multi-pred PHI incoming seeding behind owner-local helpers, so `lower_function(...)` no longer mixes prepass graph scan details inline, with proof pinned by `test_function_lower_phi_prepass.py`
      - landed B3d function-lower-if-merge slice: the same owner now keeps if-merge ret-PHI incoming seeding behind owner-local helpers, so `lower_function(...)` no longer mixes `plan_ret_phi_predeclare(...)` expansion and resolver sync inline, with proof pinned by `test_function_lower_if_merge_prepass.py`
      - landed B3d function-lower-loop-prepass slice: the same owner now keeps loop-prepass gate/debug handling behind owner-local helper `_run_loop_prepass(...)`, so `lower_function(...)` no longer mixes `detect_simple_while(...)` gate and trace inline, with proof pinned by `test_function_lower_loop_prepass.py`
      - landed B3d function-lower-ordering slice: the same owner now keeps entry-block selection and reverse-postorder/dominator/reachable computation behind owner-local helpers `_determine_entry_block_id(...)` and `_compute_lower_order(...)`, so `lower_function(...)` no longer mixes CFG ordering details inline, with proof pinned by `test_function_lower_ordering.py`
      - landed B3d function-lower-phi-ordering slice: the same owner now keeps strict/debug PHI ordering verification behind owner-local helper `_enforce_phi_ordering_contract(...)`, so `lower_function(...)` no longer mixes PHI postcondition reporting inline, with proof pinned by `test_function_lower_phi_ordering_tail.py`
      - landed B3d function-lower-finalize-tail slice: the same owner now keeps `finalize_phis -> lower_terminators -> phi-ordering contract -> terminator safety -> hot summary` orchestration behind owner-local helper `_run_finalize_tail(...)`, so `lower_function(...)` no longer mixes finalize-tail sequence inline, with proof pinned by `test_function_lower_finalize_tail.py`
      - landed B3d function-lower-signature slice: the same owner now keeps function signature policy and module reuse behind owner-local helpers `_build_function_type(...)` and `_get_or_create_function(...)`, so `lower_function(...)` no longer mixes arity policy and function lookup inline, with proof pinned by `test_function_lower_signature.py`
      - landed B3d function-lower-param-map slice: the same owner now keeps explicit-param vs heuristic ValueId binding behind owner-local helpers `_collect_param_candidate_value_ids(...)` and `_map_function_params_to_vmap(...)`, so `lower_function(...)` no longer mixes param binding scan details inline, with proof pinned by `test_function_lower_param_map.py`
      - landed B3d function-lower-cfg-scaffold slice: the same owner now keeps predecessor-map build, basic-block append, and block-id indexing behind owner-local helpers `_build_predecessor_map(...)`, `_create_basic_blocks(...)`, and `_index_blocks_by_id(...)`, so `lower_function(...)` no longer mixes CFG scaffold loops inline, with proof pinned by `test_function_lower_cfg_scaffold.py`
      - landed B3d function-lower-context-setup slice: the same owner now keeps per-function state reset and context binding behind owner-local helpers `_reset_function_lower_state(...)` and `_create_function_context(...)`, so `lower_function(...)` no longer mixes builder reset and context wiring inline, with proof pinned by `test_function_lower_context_setup.py`
      - landed B3d function-lower-resolver-seed slice: the same owner now keeps value-type metadata load and resolver fact seeding behind owner-local helpers `_load_value_types_metadata(...)` and `_seed_resolver_fact_sets(...)`, so `lower_function(...)` no longer mixes metadata/fact initialization inline, with proof pinned by `test_function_lower_resolver_seed.py`
      - landed B3d binop-route slice: `src/llvm_py/instructions/binop.py` now keeps `+` route policy behind file-local helpers for explicit dst-hint decode, operand string-fact detection, tagged-string fallback, string-tag collection, and op-alias normalization, so `lower_binop(...)` no longer mixes string-vs-integer route selection inline, with proof pinned by `test_binop_route_policy.py` and `test_binop_numeric_resolution.py`
      - landed B3d binop-entry slice: the same owner now keeps i64 operand resolve/canonicalize and textual-op alias normalization behind file-local helpers, so `lower_binop(...)` enters through `resolve operands -> normalize op -> route` orchestration with proof pinned by `test_binop_numeric_resolution.py`
      - landed B3d binop-concat slice: the same owner now keeps string-handle materialization, `any.toString_h` bridge, module-function ensure, and `concat_hh/concat3_hhh` dispatch behind file-local helpers, so `lower_binop(...)` no longer mixes concat handle prep and concat dispatch inline, with proof pinned by `test_binop_concat_helpers.py`, `test_binop_string_partial_tag.py`, and `test_strlen_fast.py`
      - landed B3d binop-int-float slice: the same owner now keeps numeric meta-kind decode, raw-or-resolved operand pickup, float operand coercion, and `fadd` emission behind file-local helpers, and `lower_binop(...)` now routes `+` through string/int-float checks before i64 canonicalization so double constants no longer spuriously hit `nyash.float.unbox_to_f64`, with proof pinned by `test_binop_int_float_promotion.py`
      - landed B3d binop-numeric-tail slice: the same owner now keeps i64 pointer coercion, expr-cache state/decode, cache-hit reuse, arithmetic-tail dispatch, vmap-trace, and result store behind file-local helpers, so `lower_binop(...)` no longer mixes numeric expr-cache orchestration and arithmetic tail emission inline, with proof pinned by `test_binop_numeric_tail.py`
      - landed compiled-stage1 surrogate shrink first slice: `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` now keeps compile-path decode, compile opts, and link-arg decode behind owner-local helpers while `module_string_dispatch.rs` continues to see only `try_dispatch(...)`
      - landed compiled-stage1 surrogate shrink second slice: the same owner now keeps backend route match and compile/link execute-error tails behind `match_route(...)`, `dispatch_route(...)`, and `finish_*_result(...)`, while parent dispatch still sees only `try_dispatch(...)`
      - landed compiled-stage1 surrogate shrink third slice: the same owner now keeps compile/link payload decode and execution behind request helpers `decode_*_request(...)` and `execute_*_request(...)`, leaving `handle_*` as decode -> execute -> finish only
      - latest compiled-stage1 residue shave: `decode_compile_obj_request(...)` is now primary-arg only, so the old arg2 rescue tail is gone and the compile route stays strict to the incoming MIR path handle
      - next backend front is no longer launcher caller cutover; it stays pinned to any remaining compiled-stage1 surrogate shrink and then the next B3d analysis/support row
      - next B3 front is no longer one Python bucket; after the resolver/type-facts, phi-manager, mir-analysis, phi-wiring analysis/tagging/finalize/selection, values-dominance, function-lower-prepass/if-merge/loop-prepass/ordering/phi-ordering/finalize-tail/signature/param-map/cfg-scaffold/context-setup/resolver-seed, and binop route/entry/concat/int-float/numeric-tail rows it is pinned to the next smaller owner seam outside `function_lower.py` unless another exact disappearing leaf appears first
      - restart handoff: safest next slices are `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` temporary residue shrink if another exact disappearing leaf appears, otherwise the next nearby owner seam should be chosen outside `function_lower.py` because its setup/tail buckets are now substantially demoted
      - latest B0 tightening: `crates/nyash-llvm-compiler/src/main.rs` now keeps harness-path resolution, object-output resolution, input temp/normalize ownership, compile-mode diagnostics, and emit finalize output behind same-file helpers `resolve_harness_path(...)`, `resolve_object_output_path(...)`, `prepare_input_json_path(...)`, `maybe_dump_input_json(...)`, `emit_preflight_shape_hint(...)`, `emit_compile_output(...)`, and `finalize_emit_output(...)`; top-level route order dispatches through `run_dummy_mode(...)` / `run_compile_mode(...)`, and `Boundary` / `Native` routes no longer resolve the Python harness path unless the explicit `Harness` keep lane is selected
      - latest B0 tightening: `src/runner/modes/common_util/exec.rs` now keeps lib/bin MIR JSON emit + ny-llvmc EXE launch behind shared helper `emit_json_and_run_ny_llvmc_emit_exe(...)`
      - latest B0 tightening: `src/runner/modes/llvm/harness_executor.rs` now keeps runtime-state log, harness gate, ny-llvmc emit, and executable run behind same-file helpers `log_harness_runtime_state(...)`, `ensure_harness_requested(...)`, `emit_executable_via_ny_llvmc(...)`, and `run_emitted_executable(...)`
      - latest BE0-min2b boundary-default slice: `crates/nyash-llvm-compiler/src/main.rs` now defaults `--driver` to `boundary` instead of `harness`, new `crates/nyash-llvm-compiler/src/boundary_driver.rs` routes default object/exe emission through the C ABI FFI bridge, and `lang/c-abi/shims/hako_aot_shared_impl.inc` now uses `--driver boundary` for its compile command while unsupported boundary shapes replay the keep lane from `lang/c-abi/shims/hako_llvmc_ffi.c` without recursive `boundary -> hako_aot -> ny-llvmc` loops
      - latest BE0-min2b keep-lane isolation: `crates/nyash-llvm-compiler/src/boundary_driver.rs` now hides FFI library open / symbol lookup behind `with_compile_symbol(...)` / `with_link_symbol(...)`, and `lang/c-abi/shims/hako_llvmc_ffi.c` now parks the pure compile branch behind `compile_json_compat_pure(...)`, so the exported default route reads as `hako_aot` forwarder while pure-lowering stays compat-only
      - latest BE0-min2c pure-first boundary slice: default boundary compile now enables the pure C subset first through caller-side recipe ownership from `crates/nyash-llvm-compiler/src/boundary_driver.rs`, `src/host_providers/llvm_codegen.rs`, and `.hako` `BackendRecipeBox`; `.hako` daily compile passes explicit `compile_json_path(..., "", "pure-first", "harness")` payload and Rust transport mirrors those names to env only at the C boundary handoff, while `lang/c-abi/shims/hako_llvmc_ffi.c` owns recursion-safe forwarders for compile/link fallback; supported v1 seeds `apps/tests/mir_shape_guard/ret_const_min_v1.mir.json` and `apps/tests/hello_simple_llvm_native_probe_v1.mir.json` are now locked by `tools/smokes/v2/profiles/integration/apps/{phase29ck_boundary_pure_first_min.sh,phase29ck_boundary_pure_print_min.sh}` to emit without relying on `NYASH_NY_LLVM_COMPILER`
      - latest BE0-min2d direct compat-keep slice: the same `lang/c-abi/shims/hako_llvmc_ffi.c` pure-first lane now replays unsupported compile shapes directly through `ny-llvmc --driver harness` instead of re-entering `hako_aot_compile_json(...)`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_compat_keep_min.sh` pins `apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json` as the current unsupported compat-keep seed
      - latest BE0-min2e boundary-command slice: `lang/c-abi/shims/hako_aot_shared_impl.inc` now builds compile commands with `ny-llvmc --driver boundary`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_forwarder_min.sh` pins the default `hako_llvmc_compile_json` forwarder path so callers without a backend recipe still reach the boundary-owned command route
      - latest BE0-min2f recipe-transport slice: recipe-aware callers now prefer the explicit `hako_llvmc_compile_json_pure_first` export, while `hako_llvmc_compile_json` stays the generic forwarder / historical compat entry; this removes daily pure-first route selection from the generic C symbol
      - latest BE0-min2f pure-string-length slice: `lang/c-abi/shims/hako_llvmc_ffi.c` now accepts a narrow ASCII-literal `StringBox.length/size` v1 seed inside the boundary-owned pure-first lane, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_length_min.sh` locks `apps/tests/mir_shape_guard/string_length_ascii_min_v1.mir.json` to emit without relying on `NYASH_NY_LLVM_COMPILER`
      - latest BE0-min2g pure-runtime-data-length slice: the same `lang/c-abi/shims/hako_llvmc_ffi.c` pure-first lane now also accepts `RuntimeDataBox.length/size` when the receiver is a `StringBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_length_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_string_length_ascii_min_v1.mir.json` as the first narrow RuntimeDataBox method-shaped seed
      - latest BE0-min2h pure-string-indexof slice: the same boundary-owned pure-first lane now accepts narrow ASCII-literal `StringBox.indexOf/1`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_string_indexof_min.sh` locks `apps/tests/mir_shape_guard/string_indexof_ascii_min_v1.mir.json` to emit without relying on `NYASH_NY_LLVM_COMPILER`
      - latest BE0-min2i pure-runtime-data-array-length slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.length/size` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_length_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_array_length_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
      - latest BE0-min2j pure-runtime-data-map-size slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.length/size` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_size_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_map_size_min_v1.mir.json` so the harness keep surface shrinks one RuntimeDataBox collection method shape at a time
      - latest BE0-min2k pure-runtime-data-map-has slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.has` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_has_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_map_has_missing_min_v1.mir.json`
      - latest BE0-min2l pure-runtime-data-map-get slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.get` when the receiver is a `MapBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_map_get_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_map_get_missing_min_v1.mir.json`
      - latest BE0-min2m pure-runtime-data-array-push slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.push` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_push_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json`
      - latest BE0-min2n pure-runtime-data-array-has slice: the same generic boundary-owned pure-first lane now accepts narrow `RuntimeDataBox.has` when the receiver is an `ArrayBox`, and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_runtime_data_array_has_min.sh` locks `apps/tests/mir_shape_guard/runtime_data_array_has_missing_min_v1.mir.json`
      - by-name follow-up is now split out as `phase-29cl` and must stay narrow to kernel/plugin/backend boundary retirement; do not repoint `phase-29ce` frontend fixture-key/by-name history there
      - landed `phase-29cl / BYN-min1`: `tools/checks/phase29cl_by_name_mainline_guard.sh` locks the `nyash.plugin.invoke_by_name_i64` owner set, and `tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh` replays the lock together with backend proof
    - exact next follow-up:
      - runtime proof owner evidence is now pinned by `docs/development/current/main/phases/phase-29ck/P4-RUNTIME-PROOF-OWNER-BLOCKER-INVENTORY.md`
      - landed runtime-proof slices:
        - `vm-hako` subset-check now allows `newbox(LlvmBackendBox)`
        - `.hako VM` runtime now has narrow `LlvmBackendBox.compile_obj/1` and `link_exe/3` helpers
        - backend boxcall helpers now route through owner-local helper methods that lower to canonical `Callee::Extern(env.codegen.*)` shape
        - regular Rust VM no longer needs receiver-less `hostbridge.extern_invoke` or placeholder `newbox(hostbridge)` for the phase-29ck proof path
        - acceptance smoke is `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
        - runtime-proof keep env is `NYASH_LLVM_USE_CAPI=1 HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
        - `.hako` daily compile now passes explicit `compile_json_path(..., "", "pure-first", "harness")`; transport layers may still mirror the same names via `HAKO_BACKEND_COMPILE_RECIPE` / `HAKO_BACKEND_COMPAT_REPLAY` at the C boundary
        - `HAKO_CAPI_PURE=1` is now compat-only historical alias and is not required by the phase-29ck `.hako VM` proof
        - explicit compat-pack entry is `tools/selfhost/run_compat_pure_pack.sh` / `tools/selfhost/run_compat_pure_selfhost.sh`; old script names are wrappers only
        - phase2120 pure canaries now route through `tools/smokes/v2/profiles/integration/core/phase2120/boundary_pure_helper.sh -> ny-llvmc --driver boundary`; do not reopen the retired direct `hostbridge.extern_invoke("env.codegen", ...)` caller lane here
      - next runtime-proof slice is promotion/cleanup after compat-pack separation, not VM blocker inventory
      - post-B1/B3 cleanup is queued as `phase-29cl` (`by_name.rs` / `module_string_dispatch.rs` caller-cutover closeout), separate from `phase-29ce`
- rule:
  - この pointer は current blocker を置き換えない。
  - immediate blocker は引き続き pure `.hako`-only hakorune build の compiler authority removal である。

## Current Blocker (SSOT)

- primary blocker:
  - exe-optimization lane now has a fresh stable baseline after the Rust stop line
  - current exact issue is to pin which remaining `kilo` / `micro kilo` leaf is still bridge/helper-density bound versus just noise
  - if the stable baseline already keeps `ratio_c_aot >= 0.95` with `aot_status=ok`, that lane can move to monitor-only; otherwise continue the lane and trim the exact hot leaf only
  - likely remaining hotspot family from the latest inventory is `TLS / LocalKey::with`, `array_get_hi`, and `find_substr_byte_index`
- current exact issue:
  1. refresh the stable `kilo_kernel_small_hk` comparison against C / Python / Hako
  2. run the micro ladder and rank the three leaves by `ratio_cycles` and `ratio_instr`
  3. if one leaf is still bridge/helper-density bound, trial `@hint(inline)` or a C bridge split on that exact leaf only
  4. the AOT contract blockers for `array_get_hi_bool_returns_i64_contract` and `substring_hii_view_materialize_boundary_contract` are now fixed in `crates/nyash_kernel/src/plugin/value_codec/decode.rs`
  5. hot bridge trace env probes are now cached in `crates/nyash_kernel/src/hako_forward_bridge.rs` and `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
  6. the next blocker is whichever micro leaf stays thick after fresh measurement
  7. latest AOT asm probe says `indexof_line` is still dominated by env/fallback guard + handle registry (`std::env::_var_os`, `LocalKey::with`, `with_str_pair`, `with_handle`) while `substring_concat` is still dominated by env/fallback guard + allocation (`std::env::_var_os`, `substring_hii`, `concat3_hhh`, `Registry::alloc`, `BoxBase::new`)
- stop line:
  - do not reopen Rust thinning or llvmlite migration for this wave
  - C bridge stays transport-only
  - `@hint(inline)` is advisory only, not a workaround
- acceptance:
  - fresh stable baseline recorded
  - micro ladder reproducible
  - any change is locked by the parity/micro smoke that covers the touched leaf
- do not do yet:
  - new `id-name` style intermediate contract
  - more bootstrap `stageN` extension without a fresh mismatch
- active owner buckets:
  - `src/llvm_py/instructions/boxcall.py`
  - `src/llvm_py/instructions/mir_call/method_fallback_tail.py`
  - `crates/nyash-llvm-compiler/src/main.rs`
  - `crates/nyash-llvm-compiler/src/native_driver.rs`
  - `src/host_providers/llvm_codegen.rs`
  - `lang/c-abi/shims/hako_aot_shared_impl.inc`
  - `lang/c-abi/shims/hako_llvmc_ffi.c`
  - `tools/llvmlite_harness.py`
  - `src/llvm_py/llvm_builder.py`
- final-owner reminder:
  - `.hako` 側は `LlvmBackendBox -> env.codegen.*` で止まる
  - final backend daily owner は Core C ABI `hako_aot` boundary
  - backend meaning/policy は `.hako` owner へ寄せ、C は export/transport だけを持つ
  - `llvm_codegen.rs` / `main.rs` / `native_driver.rs` は順に thin glue / wrapper / canary へ後退させる
- success condition for the current wave:
  - `by-name` is compat-only / no longer a daily mainline owner
  - `llvmlite` is explicit compat/probe keep only
  - `native_driver` is explicit bootstrap/canary keep only
  - the effective default exe/object route avoids both by default

## Large-Grain LlvmLite Migration Board (2026-03-17)

- goal:
  - move the daily LLVM route off `llvmlite` without inventing a third ABI or promoting `native_driver.rs` to final owner
  - keep final shape fixed to `.hako -> LlvmBackendBox -> hako_aot -> backend helper`
- stop rule:
  - stop demotion work when `llvmlite` is compat/canary keep only
  - do not delete `tools/llvmlite_harness.py` / `src/llvm_py/**` until caller docs, runner route, and acceptance scripts all read them as keep-only
  - do not delete Rust / llvmlite lanes from this repo until source + artifact preservation is completed in an external archive repo
- remaining owner buckets by grain:
  1. thin backend boundary hardening
     - exact paths:
       - `lang/src/shared/backend/llvm_backend_box.hako`
       - `lang/c-abi/include/hako_aot.h`
       - `lang/c-abi/shims/{hako_aot.c,hako_diag_mem_shared_impl.inc,hako_aot_shared_impl.inc,hako_llvmc_ffi.c}`
     - role:
       - keep `MIR(JSON path) -> object path -> exe path` contract single-sourced
       - absorb command/log/resolve/error-projection cleanup here, not in runner callers
       - become the final daily owner that `llvm_codegen.rs` only forwards into, not the other way around
       - fix the architectural split as `.hako policy/recipe owner + transport-only C substrate`
       - reduce `hako_llvmc_ffi.c -> ny-llvmc --driver harness` fallback reliance from `ret_const_min_v1` upward until `llvm_codegen.rs` can stay boundary-first without reopening Rust/CLI ownership
       - fixed order:
         - first keep widening boundary-owned compile coverage in `lang/c-abi/shims/hako_llvmc_ffi.c` for narrow pure seeds
        - landed: `.hako` recipe seam now exists as `lang/src/shared/backend/backend_recipe_box.hako`, which owns the caller-side compile recipe preflight
        - landed: `.hako` daily compile now passes explicit recipe payload into `env.codegen.compile_json_path(...)`; Rust transport mirrors that payload to env only at the boundary handoff
        - landed: `.hako` route profile now also names `acceptance_policy=boundary-pure-seed-matrix-v1`, so the current pure/compat acceptance basis is visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=ret-const-v1`, so the narrow `ret_const_min_v1` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=hello-simple-llvm-native-probe-v1`, so the narrow `hello_simple_llvm_native_probe_v1` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-array-get-missing-v1`, so the narrow `RuntimeDataBox.get(ArrayBox missing index)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-string-length-ascii-v1`, so the narrow `RuntimeDataBox.length(StringBox)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-array-length-v1`, so the narrow `RuntimeDataBox.length(ArrayBox)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-array-push-v1`, so the narrow `RuntimeDataBox.push(ArrayBox)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-map-size-v1`, so the narrow `RuntimeDataBox.length(MapBox)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-array-has-missing-v1`, so the narrow `RuntimeDataBox.has(ArrayBox missing index)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-map-has-missing-v1`, so the narrow `RuntimeDataBox.has(MapBox missing key)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=runtime-data-map-get-missing-v1`, so the narrow `RuntimeDataBox.get(MapBox missing key)` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=string-indexof-ascii-v1`, so the narrow `StringBox.indexOf/1` evidence row stays visible at the policy owner before any transport handoff
        - landed: `.hako` route profile now also names `acceptance_case=string-length-ascii-v1`, so narrow `StringBox.length/size` evidence stays visible at the policy owner before any transport handoff
        - landed: Rust VM direct `env.codegen.compile_json_path` / `emit_object` globals now delegate back to `extern_provider.rs`, so compile payload decode truth is no longer duplicated in `handlers/calls/global.rs`
         - landed: recipe-aware daily transport now prefers the explicit pure-first FFI export instead of asking the generic C export to decide that route
         - then move unsupported compile replay and seed/route policy out of `lang/c-abi/shims/hako_llvmc_ffi.c`, leaving it as export/marshal glue
         - landed: `lang/c-abi/shims/hako_aot_shared_impl.inc` compile command now uses explicit `--driver boundary`
         - next focus is no longer command repointing or C micro-thinning; it is moving pure-seed / route / compat classification into `lang/src/shared/backend/backend_recipe_box.hako` while shrinking the remaining `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness` compat surface
         - landed: boundary-owned compile coverage now includes `RuntimeDataBox.{get(MapBox),get(ArrayBox),push(ArrayBox),has(ArrayBox)}` for narrow missing-key/index seeds
      - exact next slice: make `BackendRecipeBox.compile_route_profile(...)` the first owner of broader pure/compat recipe classification, then widen from the landed `RuntimeDataBox` collection seeds to broader method-loop packs only when that recipe seam needs new evidence; route-profile shape and owner names are now fixed by `docs/development/current/main/design/backend-recipe-route-profile-ssot.md`, and the current visible evidence rows now include `acceptance_case=ret-const-v1`, `acceptance_case=hello-simple-llvm-native-probe-v1`, `acceptance_case=runtime-data-array-get-missing-v1`, `acceptance_case=runtime-data-string-length-ascii-v1`, `acceptance_case=runtime-data-array-length-v1`, `acceptance_case=runtime-data-array-push-v1`, `acceptance_case=runtime-data-map-size-v1`, `acceptance_case=runtime-data-array-has-missing-v1`, `acceptance_case=runtime-data-map-has-missing-v1`, `acceptance_case=runtime-data-map-get-missing-v1`, `acceptance_case=string-indexof-ascii-v1`, plus `acceptance_case=string-length-ascii-v1`
     - acceptance:
       - canonical runtime proof: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
       - acceptance pair: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_box_capi_link_min.sh`
       - acceptance pair: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_native_llvm_cabi_link_min.sh`
       - seed matrix / compat keep live in `docs/development/current/main/phases/phase-29ck/README.md` and `docs/development/current/main/phases/phase-29ck/P5-COMPAT-PURE-PACK-LOCK.md`
  2. ny-llvmc wrapper demotion
     - exact paths:
        - `crates/nyash-llvm-compiler/src/main.rs`
        - `crates/nyash-llvm-compiler/src/native_driver.rs`
     - role:
        - keep `ny-llvmc` as internal helper/wrapper, not caller-owned final boundary
       - move wording/route shape from `llvmlite harness wrapper` toward `backend helper with boundary/harness/native keeps`
       - keep the selector-level default exe/object path off both `DriverKind::Harness` and `DriverKind::Native`
       - keep `native_driver.rs` bootstrap-only; do not let it become the replacement default owner
     - acceptance:
       - default `ny-llvmc --emit obj ...` enters `DriverKind::Boundary`
       - default `ny-llvmc --emit exe ...` enters `DriverKind::Boundary`
       - explicit `--driver harness` remains replayable for compat fallback
       - default `ny-llvmc` path also avoids `native_driver.rs`
       - explicit `--driver harness` remains replayable
       - explicit `--driver native` remains replayable
       - phase docs must still read `native_driver.rs` as bootstrap seam only
  3. runner / host-provider route demotion
     - exact paths:
       - `src/runner/modes/llvm/**`
       - `src/runner/modes/common_util/exec.rs`
       - `src/host_providers/llvm_codegen.rs`
     - role:
       - make runner-side daily route follow backend-boundary default
       - shrink `src/host_providers/llvm_codegen.rs` to normalization/path/env glue only once boundary-first compile no longer needs wrapper fallthrough except explicit keep lanes
       - keep explicit `{llvmlite|ny-llvmc}` selection as compat/probe keep only
       - do not let route glue silently auto-fallback back into `llvmlite`
     - acceptance:
       - `cargo check --bin hakorune`
       - backend app smokes stay green with default route
       - explicit `HAKO_LLVM_EMIT_PROVIDER={llvmlite|ny-llvmc}` remains opt-in only
  4. Python llvmlite owner demotion
     - exact paths:
       - `tools/llvmlite_harness.py`
       - `src/llvm_py/llvm_builder.py`
       - `src/llvm_py/{mir_reader.py,build_ctx.py,build_opts.py,resolver.py,mir_analysis.py,phi_manager.py,type_facts.py,phi_placement.py}`
       - `src/llvm_py/instructions/**`
     - role:
       - continue reducing Python mainline ownership until it is clearly compat/canary only
       - keep the compat harness on narrow library seams, not CLI re-entry or argv mutation
       - do not mix this with backend-boundary docs; this is downstream demotion work
     - acceptance:
       - Python/llvmlite route is still replayable for probe/canary
       - daily route docs and runner stop-points no longer depend on it
- route inventory:
  - already daily-off:
    - visible `.hako` backend caller route stops at `LlvmBackendBox` / `env.codegen.*`
    - host-provider explicit `HAKO_LLVM_EMIT_PROVIDER={llvmlite|ny-llvmc}` is opt-in keep only
    - host-provider default object path now tries direct C ABI boundary before `ny-llvmc`
    - `tools/llvmlite_harness.py` no longer re-enters `llvm_builder.py` via `runpy`
  - still in-path:
    - `ny-llvmc` default `DriverKind::Boundary` and `llvm_codegen.rs` boundary-first compile still allow unsupported shapes to fall through `lang/c-abi/shims/hako_llvmc_ffi.c -> ny-llvmc --driver harness`
    - explicit keep envs `HAKO_LLVM_EMIT_PROVIDER={llvmlite|ny-llvmc}` / `NYASH_LLVM_USE_HARNESS=1`
    - Python keep owners under `tools/llvmlite_harness.py` + `src/llvm_py/**`
    - `native_driver.rs` remains the only non-llvmlite non-boundary object/exe path, but it is still bootstrap-only and must not be promoted to default
- fixed order:
  1. finish thin backend boundary hardening
  2. keep `ny-llvmc` default object/exe route on `DriverKind::Boundary` and shrink implicit harness reliance behind explicit compat fallback only
  3. verify runner/host-provider daily route stays off implicit `native_driver` and off direct `Harness` selection
  4. continue Python owner demotion until it is explicit compat/canary keep
  5. only then reconsider deleting any keep route
- do not do:
  - do not reopen `native_driver.rs` as final owner
  - do not add a new intermediate ABI or hidden env for migration convenience
  - do not silently route native failure into `llvmlite` fallback
  - do not treat backend-zero completion as permission to delete Rust / llvmlite immediately

- compiler lane: `phase-29bq / none`（active: failure-driven reopen only）
  - current blocker: `none`
  - reopen condition: `emit_fail > 0` または `route_blocker > 0`
  - lane A mirror sync helper:
    - `bash tools/selfhost/sync_lane_a_state.sh`
  - task SSOT:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
  - done: `JIR-PORT-00`（Boundary Lock, docs-first）
  - done: `JIR-PORT-01`（Parity Probe）
  - done: `JIR-PORT-02`（if/merge minimal port）
  - done: `JIR-PORT-03`（loop minimal port）
  - done: `JIR-PORT-04`（PHI / Exit invariant lock）
  - done: `JIR-PORT-05`（promotion boundary lock）
  - done: `JIR-PORT-06`（monitor-only boundary lock）
  - done: `JIR-PORT-07`（expression parity seed lock: unary+compare+logic）
  - next: `none`（failure-driven reopen only）
- runtime lane: `phase-29y / parked` current blocker: `none (parked; reopen only if a new exact vm-hako blocker appears)`
  - fixed order SSOT:
    - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- compiler pipeline lane: `hako-using-resolver-parity / monitor-only`
  - SSOT:
    - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- de-rust selfhost orchestration lane: `phase-29cc / monitor-only`
  - SSOT:
    - `docs/development/current/main/phases/phase-29cc/README.md`
    - `docs/development/current/main/design/de-rust-scope-decision-ssot.md`
    - `docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md`
  - current aftercare:
    - `DRC-01` / `DRC-02` / `DRC-03` / `DRC-04`: done
    - `DRC-05` / `DRC-06`: monitor-only
    - `DRC-08`: top-level closeout done
    - `DRC-07`: optional (`docs/private` separate repo)
  - post-closeout follow-up:
    - `docs/development/current/main/phases/phase-29ce/README.md`
    - `docs/development/current/main/phases/phase-29cf/README.md`
    - `docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md`
    - `docs/development/current/main/phases/phase-29cg/README.md`
  - current follow-up:
    - `phase-29ce / accepted`
    - `phase-29cf / accepted monitor-only`
    - `phase-29cg / accepted probing`
    - `VM fallback compat lane: explicit compat keep`
    - `bootstrap boundary reduction: future-wave target`
- perf lane: `phase-21.5 / monitor-only`
  - SSOT:
    - `docs/private/roadmap/phases/phase-21.5/PLAN.md`
