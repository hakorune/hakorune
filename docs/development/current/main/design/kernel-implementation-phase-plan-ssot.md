---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: stage axis と owner axis を混線させずに、kernel authority wave の実装フェーズ順を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
  - lang/README.md
  - lang/src/runtime/README.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/kernel/README.md
  - lang/src/runtime/meta/README.md
---

# Kernel Implementation Phase Plan (SSOT)

## Summary

- stage reading:
  - `stage0` = Rust bootstrap / recovery keep
  - `stage1` = bridge / proof line
  - `stage2-mainline` = daily `.hako` mainline / current distribution lane
  - `stage2+` = final `.hako` umbrella / distribution target
- owner reading:
  - `.hako` owns meaning / policy / route / acceptance / control
  - `.inc` is thin shim / boundary artifact (not a semantic owner noun)
  - native keeps metal / substrate (ABI/alloc/GC/TLS/atomic/backend emission)
- replacement reading:
  - `K0` = all-Rust hakorune
  - `K1` = `.hako kernel` migration stage
  - `K2` = `.hako kernel` mainline / `zero-rust` daily-distribution stage
    - `K2-core` = `RawArray first`
    - `K2-wide` = `RawMap second + capability widening + metal review`
  - task packs (`boundary lock`, semantic owner swap, `RawArray`, `RawMap`, capability widening, metal keep shrink) stay separate from `K-axis`
  - `RuntimeDataBox` stays facade-only across both `K1` and `K2`
- visible engineering reading:
  - `K0 -> K1 -> K2`
  - `K2-core` is the first substrate task pack inside `K2`
- current active order:
  - `stage / docs / naming` fixation
  - `K1 done-enough` stop-line fixation
  - `K2-core` accepted stop-line
  - `K2-wide` next structural follow-up
  - `zero-rust` default operationalization
- this document is the canonical rough task-order SSOT for the kernel replacement line.
- do not create a separate "rough order" SSOT unless this file becomes structurally overloaded.

This SSOT is the canonical phase-plan entry for the collection-first `K1` wave and the `K2` substrate handoff.

Post-collection return is owned by `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`.

## Fixed Boundaries

- `lang/src/runtime/collections/**` is the current ring1 collection semantics owner frontier for this wave.
- `lang/src/runtime/kernel/**` is runtime behavior/policy owner (string search, numeric loops, etc).
- `lang/src/runtime/meta/**` owns compiler semantic tables only.
- `lang/src/runtime/host/**` stays transport only.
- `RuntimeDataBox` stays facade/protocol only and must not regrow as collection owner during `K2-core` or `K2-wide`.

Rule:
- do not interpret "kernel .hako-ization" as "native zero" or "substrate wholesale rewrite".

## Task-Order Ownership

- coarse kernel-replacement order lives here:
  1. `K1 done-enough` stop-line
  2. `K2-core` accepted stop-line
  3. `K2-wide` next structural follow-up
  4. `zero-rust` default operationalization
- `CURRENT_TASK.md` is the restart anchor only.
- `15-Workstream-Map.md` is the one-screen operational mirror only.
- detailed slice history stays in `docs/development/current/main/phases/**`.
- therefore a separate top-level SSOT for the same rough order is not needed right now.

## Phase Order

### 1. `K1` Array phase

Goal:
- visible `ArrayBox` method semantics are `.hako` owned (policy/contract/orchestration).
- Rust remains raw substrate/compat (slot load/store, reserve/grow, layout, handle/cache).
- the phase closes only after the `.hako` array path is compared against the current Rust array baseline on the same `kilo_micro_array_getset` family and remains in an acceptable band.
- syntax audit note: the current `.hako` array/collections lane does not require the metadata annotation lane; canonical surface is `@rune`, legacy `@hint/@contract/@intrinsic_candidate` stay compat aliases, and this lane is not an Array-phase blocker.
- surfaced v1 syntax gap remains the selfhost compiler `{ ident: expr }` / BlockExpr migration note; it is outside this collection owner lane.

Stop line:
- docs/readmes/smokes describe `ArrayBox` semantics without naming Rust helpers as meaning owners.
- array path stays in `runtime/collections` (do not force-push into `runtime/kernel/array/` unless a concrete trigger fires).

### 2. `K1` Map phase

Goal:
- visible `MapBox` method semantics are `.hako` owned (policy/contract/orchestration).
- Rust remains raw substrate/compat (probe/rehash, slot load/store, layout, handle/cache).

Stop line:
- docs/readmes/smokes describe `MapBox` semantics without naming Rust helpers as meaning owners.
- `nyash.map.entry_count_i64` is the daily raw observer seam; compat aliases such as `nyash.map.entry_count_h` stay boundary-deepen tasks, not owner logic.
- do not describe this as `Map parked`; semantic `MapBox` work is `K1 done-enough`, while `RawMap` substrate work is deferred to `K2-wide`.

### 3. `K1` RuntimeData cleanup phase

Goal:
- `RuntimeDataBox` stays protocol/facade only.
- it must not become a collection semantics owner for array/map.

Stop line:
- runtime-data dispatch remains narrow and explicit.
- no doc suggests that `RuntimeDataBox` is the "collection owner".

### 4. `K2` substrate task packs

#### `K2-core`

Goal:
- `RawArray` becomes the first truthful `.hako substrate module` daily owner.
- the pilot stays same-boundary and capability-backed.
- the pilot does not redefine stage vocabulary or packaging.

Acceptance lock:
- `hako.abi / hako.value_repr / ownership-layout / fail-fast verifier` are the contract baseline.
- `RuntimeDataBox` stays facade/protocol only while `RawArray` is the daily owner.
- the pilot can be accepted without widening `Map` or reopening `K1` owner scope.

Smoke / evidence gate:
- `runtime_data_invalid_handle_returns_zero`
- `runtime_data_array_round_trip_keeps_rawarray_contract`
- `runtime_data_array_has_keeps_runtime_facade_fail_safe_contract`
- `runtime_data_array_non_i64_keys_keep_fail_safe_fallback_contract`
- `runtime_data_scalar_handle_keeps_facade_only_contract`
- `legacy_set_h_returns_zero_but_applies_value`
- `hi_hii_aliases_keep_fail_safe_contract`
- `slot_load_store_raw_aliases_keep_contract`
- `slot_append_raw_alias_keeps_contract`
- `slot_reserve_and_grow_raw_aliases_keep_length_and_expand_capacity`
- lowering/manifest drift pack:
  - `test_runtime_data_dispatch_policy`
  - `test_collection_method_call`
  - `test_boxcall_collection_policy`
  - `test_rawarray_manifest_lock`
- treat those as the explicit `K2-core` acceptance pack.

Stop line:
- `RawArray` has explicit contract baseline under `hako.abi / hako.value_repr / ownership-layout / fail-fast verifier`.
- the daily owner reading is `.hako substrate module`, not hidden Rust helper ownership.
- the pilot can be judged without reopening broad collection semantic-owner work.

#### `K2-wide`

Goal:
- open only after the `RawArray` pilot is operationally stable.
- keep `MapBox` visible semantics on the existing `.hako` owner frontier while substrate replacement is still narrow.
- absorb capability widening and metal review into the same era instead of exposing new public milestones.
- treat the internal work order as:
  1. `RawMapCoreBox` narrow substrate widening (`entry_count / cap / probe / slot_load / slot_store`)
  2. capability widening packs (`hako.atomic` -> `hako.tls` -> `hako.gc` -> `hako.osvm`)
  3. `hako_alloc` policy/state rows plus allocator/TLS/GC policy-owner widening
  4. metal keep review as truthful seam inventory + boundary-shrink planning

Stop line:
- treat `Map` as narrow façade + regression pack until the `RawArray` pilot is accepted.
- keep `RawMap second / RuntimeData facade-only` while widening.
- do not widen through ad hoc native escape hatches; widening must stay on capability modules.
- do not call this `K3`; the widening remains the second task pack inside `K2`.
- first-slice acceptance pack is:
  - Rust/kernel RawMap acceptance tests
  - MapBox lowering lock
  - raw-map ABI/substrate route lock
- current capability-row pack starts with `hako.atomic` and `AtomicCoreBox.fence_i64()`
- current helper-shaped capability-row pack is `hako.tls` via `TlsCoreBox.last_error_text_h()`
- current capability-row pack is `hako.gc` via `GcCoreBox.write_barrier_i64(handle_or_ptr)`
- current capability-row pack is `hako.osvm` via `OsVmCoreBox.reserve_bytes_i64(len_bytes)`
- current policy/state row pack after capability widening is `hako_alloc` GC trigger threshold policy
- handle reuse policy is landed below it, and no third live `hako_alloc` row is open yet
- current metal keep review pack is the truthful seam inventory + boundary-shrink planning lock, machine-owned by `k2_wide_metal_keep_inventory_guard.sh`

### 5. Regression / perf pack

Rule:
- `Array -> Map -> RuntimeData cleanup` remains a regression/evidence pack while `K2` is being prepared.
- map perf remains evidence/monitor-only while `RawMap` stays deferred in `K2-wide`.
- do not reopen broad authority expansion while route/perf evidence is active.
- any further owner migration requires a new exact blocker and a dedicated SSOT update.
- the first post-entry optimization wave is `route/perf only` on `.hako -> ny-llvmc(boundary) -> C ABI`.
- Rune optimization metadata remains `parse/noop`, and backend-active consumption is out of scope for this return wave.

## Acceptance Gates (Docs + Smokes)

The phase plan is considered "done-enough to return to perf-kilo" when:

1. stage docs agree on: `stage0 keep / stage1 bridge+proof / stage2-mainline daily mainline / stage2+ umbrella`.
2. replacement docs agree on: `K0 all-Rust hakorune / K1 .hako kernel migration stage / K2 .hako kernel mainline-zero-rust daily-distribution stage`.
   - `K2-core` accepted stop-line is closed before any `K2-wide` widening.
3. owner docs agree on: `.hako authority / .inc thin shim / native metal keep`.
4. collection docs agree on: `Array phase -> Map phase -> RuntimeData cleanup phase`.
5. daily proof locks remain green:
  - `runtime_data_invalid_handle_returns_zero`
  - `runtime_data_array_round_trip_keeps_rawarray_contract`
  - `runtime_data_array_has_keeps_runtime_facade_fail_safe_contract`
  - `runtime_data_array_non_i64_keys_keep_fail_safe_fallback_contract`
  - `runtime_data_scalar_handle_keeps_facade_only_contract`
  - `legacy_set_h_returns_zero_but_applies_value`
  - `hi_hii_aliases_keep_fail_safe_contract`
  - `slot_load_store_raw_aliases_keep_contract`
  - `slot_append_raw_alias_keeps_contract`
  - `slot_reserve_and_grow_raw_aliases_keep_length_and_expand_capacity`
  - `test_runtime_data_dispatch_policy`
  - `test_collection_method_call`
  - `test_boxcall_collection_policy`
  - `test_rawarray_manifest_lock`
  - `runtime_data_map_get_keeps_mixed_runtime_i64_contract` stays evidence-only for the parked map lane
6. regression/perf evidence stays recorded against the current Rust baseline:
  - `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_array_getset 1 3`
  - `tools/perf/run_kilo_micro_machine_ladder.sh`
  - `tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_map_get_missing 0 1`
  - read those as regression/evidence packs while `RawArray` remains the first `K2-core` structural pilot

## Non-Goals

- no new public ABI
- no "native zero" claim
- no wholesale move of `array/map` into `runtime/kernel/{array,map}` without a trigger
- no treating `Map` optimization evidence as the next structural replacement milestone ahead of `K2-core RawArray`
