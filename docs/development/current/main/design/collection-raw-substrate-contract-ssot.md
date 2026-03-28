---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `ArrayBox` / `MapBox` の `.hako` ring1 owner と Rust raw substrate の境界を固定し、collection owner cutover の first implementation order を 1 枚で読む。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/collections/array_core_box.hako
  - lang/src/runtime/collections/array_state_core_box.hako
  - crates/nyash_kernel/src/plugin/array.rs
  - crates/nyash_kernel/src/plugin/array_index_dispatch.rs
  - crates/nyash_kernel/src/plugin/array_write_dispatch.rs
  - crates/nyash_kernel/src/plugin/handle_cache.rs
---

# Collection Raw Substrate Contract (SSOT)

## Purpose

- collection owner cutover を「Rust を速くするか」ではなく「意味 owner をどこに置くか」で固定する。
- `.hako ring1 collection core` と Rust raw substrate の境界を、名前と責務で判定できるようにする。
- `array -> map -> runtime_data cleanup` の first implementation order を、docs だけで再起動可能にする。

## 1. Final Shape

- `.hako` ring1 collection core owns:
  - `ArrayBox` / `MapBox` user-visible semantics
  - method aliases (`len/length/size`)
  - bounds / missing-key / fallback / visible error contract
  - index/key normalization
  - smoke-facing collection behavior
- Rust raw substrate owns:
  - exact allocation / reserve / growth
  - slot load/store
  - probe / rehash / bucket walk mechanics
  - downcast / handle cache / encoded value codec
  - object layout / GC barrier / ABI marshal
- `RuntimeDataBox` stays protocol / facade only:
  - route/dynamic dispatch is allowed
  - collection semantics ownership is not allowed

## 1.5 Current State

- this SSOT governs the owner/substrate boundary only; it does not define the `stage0/stage1/stage2+` axis
- owner shift is done-enough for the current phase, but not end-state complete
- `.hako` ring1 collection core is the visible owner frontier for collection semantics
- Rust still owns the raw substrate and compatibility/plugin ABI path beneath that frontier
- do not describe this phase as finished while method-shaped Rust exports still remain in the daily `.hako` path
- next lane after the collection stop-line is capability widening, not blind raw helper growth
- `phase-29ct` owns that next lane

## 2. Litmus Test

- the common optimization unit for collection work is `recipe family`, not benchmark name
- method semantics may widen by `receiver_family / method_family / scope_class / effect_profile`
- benchmark-specific proof lanes may exist, but they must not become permanent owner boundaries

### Put it in `.hako`

- method-shaped names:
  - `get`
  - `set`
  - `push`
  - `has`
  - `len`
  - `length`
  - `size`
- policy/contract questions:
  - bounds policy
  - missing-key behavior
  - index/key normalization
  - visible fallback/error contract
- anything a smoke/spec can describe without mentioning handles/layout/GC

### Keep it in Rust

- substrate-shaped names:
  - `slot_load`
  - `slot_store`
  - `reserve`
  - `grow`
  - `probe`
  - `rehash`
  - `encode`
  - `decode`
  - `downcast`
  - `cache`
- anything that must know about:
  - handle registry
  - object layout
  - allocator / raw memory
  - GC barrier / root behavior
  - ABI / backend boundary

## 3. Boundary Rule

- the boundary must sit below `array_get` / `array_set` / `map_has` / `map_get`
- the boundary may sit at raw verbs like:
  - `array_slot_load_*`
  - `array_slot_store_*`
  - `array_reserve_*`
  - `map_probe_*`
  - `map_rehash_*`
  - `map_slot_load_*`
  - `map_slot_store_*`
- transitional method-shaped Rust helpers such as:
  - `array_get_by_index`
  - `array_set_by_index_i64_value`
  - `map_*` method helpers with visible semantics
  are not the target end-state and should disappear or be renamed behind raw substrate boundaries.

## 4. First Implementation Order

### A1. Pin `.hako` Array semantics

Owners:
- `lang/src/runtime/collections/array_core_box.hako`
- `lang/src/runtime/collections/array_state_core_box.hako`

Target:
- make `.hako` the visible owner for:
  - `ArrayBox.{get,set,push,len,length,size}`
  - bounds policy
  - visible fallback/error contract
  - normalization decisions

### A2. Define Array raw substrate verbs

Owners:
- `crates/nyash_kernel/src/plugin/array.rs`
- `crates/nyash_kernel/src/plugin/array_index_dispatch.rs`
- `crates/nyash_kernel/src/plugin/array_write_dispatch.rs`
- `crates/nyash_kernel/src/plugin/handle_cache.rs`

Target:
- demote Rust array helpers to raw substrate responsibilities only
- prefer raw naming and raw contracts over method-shaped naming
- keep handle/cache/downcast/layout logic local to Rust

Current first slice:
- `crates/nyash_kernel/src/plugin/array_slot_load.rs`
- `crates/nyash_kernel/src/plugin/array_slot_store.rs`
- `array_index_dispatch.rs` / `array_write_dispatch.rs` remain as thin compatibility wrappers while raw slot verbs become the structural owner

### A3. Retarget `.hako` Array owner to raw verbs

Target:
- `.hako` calls become the only method-semantics owner
- Rust exports become storage/mechanics substrate only
- acceptance stays on existing array/provider/runtime-data smokes while the boundary moves downward

Current first slice:
- `ArrayCoreBox.get_i64` -> `nyash.array.slot_load_hi`
- `ArrayCoreBox.set_i64` -> `nyash.array.slot_store_hii`
- `ArrayCoreBox.len_i64` -> `nyash.array.slot_len_h`
- `ArrayCoreBox.push_hh` -> `nyash.array.slot_append_hh`
- legacy `nyash.array.get_hi` / `nyash.array.set_hii` stay as compatibility shell during the retarget wave
- transitional method-shaped Rust exports still visible from `.hako`:
  - none in the current daily `.hako` owner path

### M1. Repeat for Map

Owners:
- `lang/src/runtime/collections/map_core_box.hako`
- Rust `map` plugin/helpers

Target:
- `.hako` owns visible `MapBox` semantics
- Rust owns hash/probe/rehash/layout mechanics only

Current first slice:
- `MapCoreBox.try_handle(...)` is now the single handler-side visible owner frontier for `MapBox.{set,get,has,size/len/length}`
- `lang/src/vm/boxes/mir_call_v1_handler.hako` no longer carries inline `MapBox.set` fallback logic

Current second slice:
- `crates/nyash_kernel/src/plugin/map_slot_load.rs`
- `crates/nyash_kernel/src/plugin/map_slot_store.rs`
- `crates/nyash_kernel/src/plugin/map_probe.rs`
- legacy `nyash.map.{get,set,has}_*` exports stay as thin compatibility wrappers while raw `slot_load` / `slot_store` / `probe` verbs become the structural owner

Current third slice:
- `lang/src/runtime/collections/map_state_core_box.hako` now owns vm-hako-visible `MapBox.{set,get,has,getField,setField,delete,keys,clear}` stateful routing
- `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako` only delegates those method-shaped routes instead of owning inline map state semantics
- transitional method-shaped Rust export still visible from `.hako`:
  - `nyash.map.entry_count_h`

### R1. Cleanup RuntimeData

Owner:
- `lang/src/runtime/collections/runtime_data_core_box.hako`

Target:
- protocol / facade / routing only
- do not absorb array/map semantics

Current first slice:
- `crates/nyash_kernel/src/plugin/runtime_data.rs` is now a dispatch shell only
- collection-specific mechanics live in:
  - `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data_map_dispatch.rs`
- the exported `nyash.runtime_data.{get,set,has,push}_*` ABI contract stays unchanged while route ownership becomes explicit

Current second slice:
- `lang/src/runtime/collections/runtime_data_core_box.hako` now owns its own unary/binary arg decode helpers plus ABI dispatch helpers
- `lang/src/vm/boxes/mir_call_v1_handler.hako` treats `RuntimeDataBox` as one delegated branch instead of carrying per-method routing

## 4.5 Boundary-Deepen Before Perf

- before reopening raw substrate perf, first inventory and demote the transitional method-shaped Rust exports that still sit in the daily `.hako` path
- current exact transitional list:
  - `nyash.map.entry_count_h`
- landed observer demotion:
  - daily `.hako` array observer path now uses `nyash.array.slot_len_h`
  - `nyash.array.len_h` remains compatibility-only
- landed append demotion:
  - daily `.hako` array append path now uses `nyash.array.slot_append_hh`
  - arrayish runtime-data mono-route now uses `nyash.array.slot_append_hh`
  - `nyash.array.push_hh` remains compatibility-only
- landed compat/pure append retarget:
  - adapter defaults and historical pure `ArrayBox.push -> len` lowering now use `nyash.array.slot_append_hh`
  - `nyash.array.push_h` remains compatibility-only
- landed compat/pure array-get retarget:
  - adapter defaults and historical pure `ArrayBox.get` lowering now use `nyash.array.slot_load_hi`
  - `nyash.array.get_h` remains compatibility-only
- landed map observer demotion:
  - daily `.hako` map observer path now uses `nyash.map.entry_count_h`
  - `nyash.map.size_h` remains compatibility-only
- landed compat/pure map retarget:
  - adapter defaults and historical pure `MapBox.{get,set,has}` lowering now use `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh` / `nyash.map.probe_hh`
  - `nyash.map.{get_h,set_h,has_h}` remain compatibility-only
- landed append hidden-residue slice:
  - `nyash.array.slot_append_hh` now executes through `ArrayBox.slot_append_box_raw(...)`
  - compat append routes share the same raw append helper instead of visible `push()`
- landed store hidden-residue slice:
  - `nyash.array.slot_store_hii` now executes through `ArrayBox.slot_store_*_raw(...)`
  - runtime-data array set shares the same raw store helper instead of visible `try_set_index_i64*()`
- landed map hidden-residue slice:
  - `nyash.map.slot_* / probe_*` now execute through `MapBox.{get_opt_key_str,insert_key_str,contains_key_str}(...)`
  - `nyash.map.entry_count_h` now executes through `MapBox.entry_count_i64(...)`

## 4.6 Post-Stop-Line Next Lane

- after the current collection stop-line, the next fixed order is:
  - `substrate-capability-ladder-ssot.md`
  - `value-repr-and-abi-manifest-ssot.md`
- the next work is:
  - manifest-first export inventory
  - canonical runtime value classes
  - future `hako.mem` / `hako.ptr` / `hako.buf` module lock
- do not treat `array_slot_*` / `map_slot_*` growth alone as sufficient for allocator/Hakozuna migration
- kernel-side review result:
  - the new `MapBox` raw key-string helpers are acceptable as the raw seam for this slice
- landed runtime-data map hidden-residue slice:
  - `runtime_data_map_dispatch.rs` now delegates map behavior through accepted `map_slot_load_any` / `map_slot_store_any` / `map_probe_contains_any`
- current remaining work after those explicit exports:
  - active llvm-py lowering still keeps the i64-key array set path on method-shaped exports (`nyash.array.set_hih` / `nyash.array.set_hii`)
- `RuntimeDataBox` does not join that owner growth; it stays facade-only
- raw substrate perf should stay parked until this deeper boundary is fixed or these exports are explicitly accepted as the long-term substrate cut

### Current B1 taskization

1. `B1a / array-observer`
   - landed: remove `nyash.array.len_h` from the daily `.hako` path
   - daily route now targets `nyash.array.slot_len_h`
2. `B1b / array-append`
   - landed: remove `nyash.array.push_hh` from the daily `.hako` path
   - daily route now targets `nyash.array.slot_append_hh`
3. `B1c / map-observer`
   - landed: remove `nyash.map.size_h` from the daily `.hako` path
   - daily route now targets `nyash.map.entry_count_h`
4. `B1k / compat-append-retarget`
   - landed: remove `nyash.array.push_h` from adapter defaults and historical pure `ArrayBox.push -> len` lowering
   - adapter/pure route now target `nyash.array.slot_append_hh`
5. `B1m / compat-array-get-retarget`
   - landed: remove `nyash.array.get_h` from adapter defaults and historical pure `ArrayBox.get` lowering
   - adapter/pure route now target `nyash.array.slot_load_hi`
6. `B1l / compat-map-retarget`
   - landed: remove `nyash.map.{get_h,set_h,has_h}` from adapter defaults and historical pure `MapBox.{get,set,has}` lowering
   - adapter/pure route now target `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh` / `nyash.map.probe_hh`
7. `B1d / array-write-hidden-residue`
   - first slice landed: append now goes through `ArrayBox.slot_append_box_raw(...)`
   - second slice landed: store now goes through `ArrayBox.slot_store_*_raw(...)`
   - remaining work is semantic, not method-shaped API leakage
8. `B1e / map-hidden-residue`
   - landed: move visible `get_opt/set/has/size`-shaped semantics out from under `nyash.map.slot_* / probe_*`
   - inventory result: `MapBox.{get_opt_key_str,insert_key_str,contains_key_str,entry_count_i64}` is acceptable as the kernel-side raw boundary for this slice
9. `B1f / aot-prep-lowering-residue`
   - landed: retarget active `collections_hot.hako` rewrites away from method-shaped collection exports where the raw seam already exists
   - landed set:
     - `ArrayBox.get -> nyash.array.slot_load_hi`
     - `ArrayBox.push -> nyash.array.slot_append_hh`
     - `MapBox.get/set/has -> nyash.map.slot_load_* / slot_store_* / probe_*`
   - landed: adapter defaults and historical pure `ArrayBox.set` lowering now use `nyash.array.set_hih`
   - contract pin:
     - `bash tools/smokes/v2/profiles/integration/apps/phase29cm_collections_hot_raw_route_contract_vm.sh`
10. `B1g / llvm-py-lowering-residue`
   - landed: retarget active llvm-py collection lowering away from method-shaped collection exports where the raw seam already exists
   - landed set:
     - shared collection fallback now uses `nyash.array.slot_append_hh` and `nyash.map.slot_load_hh / slot_store_hhh / probe_hh`
     - direct boxcall lowering now uses `nyash.array.slot_load_hi` where the i64 raw seam exists
     - runtime-data array mono-route now uses `nyash.array.slot_load_hi` for i64-key `get`
   - keep:
     - the remaining array lowering keep moved to `B1i/B1j`
   - contract pins:
     - `python3 -m unittest src.llvm_py.tests.test_collection_method_call src.llvm_py.tests.test_runtime_data_dispatch_policy src.llvm_py.tests.test_mir_call_auto_specialize src.llvm_py.tests.test_boxcall_plugin_invoke_args src.llvm_py.tests.test_strlen_fast`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
     - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
11. `B1h / runtime-data-map-hidden-residue`
   - landed: `runtime_data_map_dispatch.rs` now delegates map behavior through accepted `map_slot_load_any` / `map_slot_store_any` / `map_probe_contains_any`
   - contract pins:
     - `cargo test -q -p nyash_kernel runtime_data_invalid_handle_returns_zero --lib`
     - `cargo test -q -p nyash_kernel runtime_data_dispatch_map_set_get_has --lib`
     - `cargo test -q -p nyash_kernel runtime_data_dispatch_map_push_missing_key_contract --lib`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
     - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
     - `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_map_provider_vm.sh`
9. `B1i / array-non-i64-lowering-residue`
   - landed first slice: active lowering now uses `nyash.runtime_data.get_hh/has_hh/set_hhh` for array non-i64 shapes
   - proven i64-key routes stay direct for now (`nyash.array.slot_load_hi`, `nyash.array.set_hih`, `nyash.array.set_hii`)
   - contract pins:
     - `python3 -m unittest src.llvm_py.tests.test_runtime_data_dispatch_policy src.llvm_py.tests.test_strlen_fast src.llvm_py.tests.test_boxcall_plugin_invoke_args`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
     - `bash tools/smokes/v2/profiles/integration/phase21_5/perf/kilo/phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh`
10. `B1j / array-i64-set-keep-inventory`
   - landed: keep the remaining active i64-key array set path as an explicit accepted substrate cut
   - accepted keep contract:
     - `nyash.array.set_hii` stays the i64-key + i64-value specialized route
     - `nyash.array.set_hih` stays the i64-key + handle/any-value fallback route
     - do not add `slot_store_hih`; it would duplicate the accepted keep without reducing the daily-path surface
   - contract pins:
     - `cargo test -q -p nyash_kernel array_runtime_data_route_hi_contract_roundtrip --lib`
     - `cargo test -q -p nyash_kernel array_runtime_data_route_hii_contract_roundtrip --lib`
     - `python3 -m unittest src.llvm_py.tests.test_strlen_fast src.llvm_py.tests.test_boxcall_plugin_invoke_args`
     - `crates/nyash_kernel/src/plugin/array_slot_store.rs`
11. `B1n / array-set-compat-retarget`
   - landed: adapter defaults and historical pure `ArrayBox.set` lowering now use `nyash.array.set_hih`
   - `nyash.array.set_h` remains compatibility-only
   - contract pins:
     - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
     - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
     - `bash tools/smokes/v2/profiles/integration/core/phase2120/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`
11. `B1r / runtime_data lock`
   - no active code task; only reopen on an exact protocol/dispatch bug

## 5. Naming Rule

- `.hako` names should stay user-visible and method-shaped
- Rust names should become metal/mechanics-shaped
- if a Rust function name can be dropped into a language spec sentence unchanged, it is probably too high-level to stay in Rust

## 6. Acceptance

- source-contract gate:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
  - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- array quick:
  - `bash tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh`
- map quick available now:
  - `bash tools/smokes/v2/profiles/quick/core/map/map_basic_get_set_vm.sh`
  - `bash tools/smokes/v2/profiles/quick/core/map/map_len_size_vm.sh`
- provider integration:
  - `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_map_provider_vm.sh`
- runtime_data guard:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`

## 7. Non-Goals

- moving collection semantics into `ring0`
- growing `RuntimeDataBox` into a collection-semantics owner
- reopening raw substrate perf work before `array` / `map` method ownership leaves Rust
- calling the phase “finished” while the daily path still crosses method-shaped Rust exports
- forcing a dedicated `lang/src/runtime/kernel/{array,map}/` module before ring1 collection core ownership is complete
