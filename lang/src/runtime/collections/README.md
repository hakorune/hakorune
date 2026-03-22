# lang/src/runtime/collections — Ring1 Collection Runtime Core

Scope: `.hako` ring1 collection core for user-visible collection semantics during the done-enough owner shift from Rust-owned semantics to Rust-owned raw substrate.

## Responsibility

- Own user-visible `ArrayBox` / `MapBox` collection semantics in `.hako` ring1.
- Provide collection-facing `.hako` routing for VM core without pushing collection semantics into ring0.
- Keep std-layer helpers (`apps/std/*`) out of VM low-level execution path.
- Delegate raw storage/primitive operations to Rust-owned substrate symbols (`nyash.array.*`, etc.) after the `.hako` layer decides method semantics; Rust keeps the raw substrate for now.

## Current Truth

- This folder is the visible owner frontier for `ArrayBox` / `MapBox` semantics.
- Current mainline still delegates primitive storage/ops to Rust-owned ABI/plugin exports; the raw substrate remains Rust-owned until the boundary deepens.
- `RuntimeDataBox` stays protocol / facade only; do not turn it into a collection-semantics owner.
- Runtime/provider current-truth and `0rust` cutover order are tracked in:
  - `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`
  - `docs/development/current/main/design/collection-raw-substrate-contract-ssot.md`

Rule:
- keep this layer ring1
- do not move collection semantics into ring0
- method-shaped verbs (`get/set/push/has/len/length/size`, normalization, visible fallback/error contract) belong here
- raw substrate verbs (`encode/decode/cache/downcast/load/store/probe/rehash/layout`) remain in Rust until the phase boundary is deeper
- future owner growth belongs to `.hako` ring1 collection/runtime, not OS-facing ring0

## Current modules

- `array_core_box.hako`
  - `get_i64(handle, idx)` / `set_i64(handle, idx, value)` / `len_i64(handle)` / `push_hh(handle, value_any)`
    -> `runtime/substrate/raw_array/raw_array_core_box.hako`
    -> `runtime/substrate/buf/buf_core_box.hako` for reserve/grow
    -> `runtime/substrate/ptr/ptr_core_box.hako`
    -> `nyash.array.slot_load_hi` / `nyash.array.slot_store_hii` / `nyash.array.slot_len_h` / `nyash.array.slot_append_hh`
  - `try_handle(seg, regs, mname)`
    -> visible owner for `ArrayBox.{set,get,push,len/length/size}` orchestration, bounds contract, and fallback
- `array_state_core_box.hako`
  - `record_push_state(...)` / `record_set_state(...)` / `get_state_value(...)`
    -> ArrayBox state bookkeeping support for the `.hako` owner
- `runtime_data_core_box.hako`
  - `try_handle(seg, regs, mname)`
    -> narrow `RuntimeDataBox.{get,set,has,push}` protocol/facade dispatch for `mir_call_v1_handler`
  - `get_hh(recv_h, key_any)` / `set_hhh(recv_h, key_any, val_any)` / `has_hh(recv_h, key_any)` / `push_hh(recv_h, val_any)`
    -> `nyash.runtime_data.*` thin extern wrapper used by the same facade
  - paired Rust route modules:
    - `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`
    - `crates/nyash_kernel/src/plugin/runtime_data_map_dispatch.rs`
    - `crates/nyash_kernel/src/plugin/runtime_data.rs` now stays dispatch-shell only
- `map_core_box.hako`
  - `try_handle(seg, regs, mname)`
    -> visible owner for `MapBox.{set,get,has,size/len/length}` orchestration and current handler-side contract
  - `size_i64(handle)` -> `nyash.map.entry_count_h`
  - `norm_key_str(raw)` -> stable MapBox key normalization for adapter-on state
  - `record_set_state(...)` / `get_state_value(...)` / `has_state_value(...)`
    -> MapBox state bookkeeping support for the `.hako` owner
  - paired Rust raw substrate:
    - `crates/nyash_kernel/src/plugin/map_slot_load.rs`
    - `crates/nyash_kernel/src/plugin/map_slot_store.rs`
    - `crates/nyash_kernel/src/plugin/map_probe.rs`
    - legacy `nyash.map.{get,set,has}_*` exports stay as thin compatibility shells above those raw verbs
- `map_state_core_box.hako`
  - `apply_set/get/has/getField/setField/delete/keys/clear(...)`
    -> vm-hako-visible MapBox stateful routing for collections ring1
  - keeps key normalization, visible missing/bad-key contract, and state bookkeeping out of `mir_vm_s0_boxcall_builtin.hako`
- `string_core_box.hako`
  - `len_i64(handle)` -> `nyash.string.len_h`
  - `try_handle(seg, regs, mname)`
    -> owns adapter-on `StringBox.length/len/size` orchestration for `mir_call_v1_handler`

## Current proof lock

- source-contract smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
- array provider smoke:
  - `tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh`
- map provider smoke:
  - `tools/smokes/v2/profiles/integration/ring1_providers/ring1_map_provider_vm.sh`
- standalone RuntimeDataBox e2e smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`

## First Cutover Order

1. `ArrayCoreBox` / `array_state_core_box.hako`
   - become the visible `ArrayBox` semantics owner
2. Rust array helpers
   - shrink to raw substrate verbs only
3. `MapCoreBox`
   - follows the same split
4. `RuntimeDataCoreBox`
  - cleanup to protocol / facade only
5. `B1`
  - landed: daily array observer route now uses `nyash.array.slot_len_h`
  - landed: daily array append route now uses `nyash.array.slot_append_hh`
  - landed: daily array `get/set/len/push` substrate hop now goes through `RawArrayCoreBox -> PtrCoreBox`
  - landed: `RawArrayCoreBox.reserve/grow` now route through `BufCoreBox`
  - landed: adapter defaults and historical pure `ArrayBox.push -> len` lowering now use `nyash.array.slot_append_hh`
  - landed: adapter defaults and historical pure `ArrayBox.get` lowering now use `nyash.array.slot_load_hi`
  - landed: `nyash.array.slot_append_hh` now executes through `ArrayBox.slot_append_box_raw(...)`
  - landed: `nyash.array.slot_store_hii` now executes through `ArrayBox.slot_store_*_raw(...)`
  - landed: daily map observer route now uses `nyash.map.entry_count_h`
  - `nyash.map.size_h` is compat-only
  - landed: adapter defaults and historical pure `MapBox.{get,set,has}` lowering now use `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh` / `nyash.map.probe_hh`
  - landed: `nyash.map.slot_* / probe_*` now execute through `MapBox.{get_opt_key_str,insert_key_str,contains_key_str}(...)`
  - worker inventory: those `MapBox` raw key-string helpers are acceptable as the kernel-side raw seam for this slice
  - landed: `collections_hot.hako` now retargets array `get/push` and map `get/set/has` to raw seams
  - landed: adapter defaults and historical pure `ArrayBox.set` lowering now use `nyash.array.set_hih`
  - `nyash.array.set_h` remains compatibility-only
  - landed: active llvm-py lowering now uses raw seams where they already exist (`array push`, `array i64 get`, `map get/set/has`)
  - landed: `runtime_data_map_dispatch.rs` now delegates map behavior through accepted `map_slot_load_any` / `map_slot_store_any` / `map_probe_contains_any`
  - landed first slice: active lowering now routes array non-i64 `get/has` and non-i64 `set` through `nyash.runtime_data.*`
  - landed accepted keep: the remaining i64-key array set route stays split as `nyash.array.set_hii` for i64/i64 and `nyash.array.set_hih` for i64-key + handle/any-value
  - keep `RuntimeDataBox` facade-only while doing so
