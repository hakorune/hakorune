---
Status: SSOT
Decision: provisional
Date: 2026-03-21
Scope: `ArrayBox` / `MapBox` の `.hako` ring1 owner と Rust raw substrate の境界を固定し、collection owner cutover の first implementation order を 1 枚で読む。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/collections/array_core_box.hako
  - lang/src/runtime/collections/array_state_core_box.hako
  - crates/nyash_kernel/src/plugin/array.rs
  - crates/nyash_kernel/src/plugin/array_index_helpers.rs
  - crates/nyash_kernel/src/plugin/array_route_helpers.rs
  - crates/nyash_kernel/src/plugin/handle_helpers.rs
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

## 2. Litmus Test

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
- `crates/nyash_kernel/src/plugin/array_index_helpers.rs`
- `crates/nyash_kernel/src/plugin/array_route_helpers.rs`
- `crates/nyash_kernel/src/plugin/handle_helpers.rs`

Target:
- demote Rust array helpers to raw substrate responsibilities only
- prefer raw naming and raw contracts over method-shaped naming
- keep handle/cache/downcast/layout logic local to Rust

Current first slice:
- `crates/nyash_kernel/src/plugin/array_slot_load.rs`
- `crates/nyash_kernel/src/plugin/array_slot_store.rs`
- `array_index_helpers.rs` / `array_route_helpers.rs` remain as thin compatibility wrappers while raw slot verbs become the structural owner

### A3. Retarget `.hako` Array owner to raw verbs

Target:
- `.hako` calls become the only method-semantics owner
- Rust exports become storage/mechanics substrate only
- acceptance stays on existing array/provider/runtime-data smokes while the boundary moves downward

Current first slice:
- `ArrayCoreBox.get_i64` -> `nyash.array.slot_load_hi`
- `ArrayCoreBox.set_i64` -> `nyash.array.slot_store_hii`
- legacy `nyash.array.get_hi` / `nyash.array.set_hii` stay as compatibility shell during the retarget wave

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

### R1. Cleanup RuntimeData

Owner:
- `lang/src/runtime/collections/runtime_data_core_box.hako`

Target:
- protocol / facade / routing only
- do not absorb array/map semantics

Current first slice:
- `crates/nyash_kernel/src/plugin/runtime_data.rs` is now a dispatch shell only
- collection-specific mechanics live in:
  - `crates/nyash_kernel/src/plugin/runtime_data_array_route.rs`
  - `crates/nyash_kernel/src/plugin/runtime_data_map_route.rs`
- the exported `nyash.runtime_data.{get,set,has,push}_*` ABI contract stays unchanged while route ownership becomes explicit

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
  - `bash tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/ring1_map_provider_vm.sh`
- runtime_data guard:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`

## 7. Non-Goals

- moving collection semantics into `ring0`
- growing `RuntimeDataBox` into a collection-semantics owner
- reopening raw substrate perf work before `array` / `map` method ownership leaves Rust
- forcing a dedicated `lang/src/runtime/kernel/{array,map}/` module before ring1 collection core ownership is complete
