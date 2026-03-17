# lang/src/runtime/collections — Ring1 Collection Runtime Core

Scope: VM low-level collection runtime wrappers for cutover phases.

## Responsibility

- Provide thin `.hako` wrappers for collection ABI vocabularies used by VM core.
- Keep std-layer helpers (`apps/std/*`) out of VM low-level execution path.
- Delegate storage/primitive operations to ABI symbols (`nyash.array.*`, etc.) without adding policy logic.

## Current Truth

- This folder is not yet the concrete collection owner.
- Current mainline still delegates primitive storage/ops to Rust-owned ABI/plugin exports.
- Runtime/provider current-truth and `0rust` cutover order are tracked in:
  - `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`

Rule:
- keep this layer thin
- do not move collection semantics into ring0
- future owner growth belongs to `.hako` ring1 collection/runtime, not OS-facing ring0

## Current modules

- `array_core_box.hako`
  - `get_i64(handle, idx)` / `set_i64(handle, idx, value)` / `len_i64(handle)`
    -> `nyash.array.get_hi` / `nyash.array.set_hii` / `nyash.array.len_h`
  - `try_handle(seg, regs, mname)`
    -> owns adapter-on `ArrayBox.{set,get,push,len/length/size}` orchestration for `mir_call_v1_handler`
  - `record_push_state(...)` / `record_set_state(...)` / `get_state_value(...)`
    -> adapter-on ArrayBox size/value-state ownership for `mir_call_v1_handler`
- `runtime_data_core_box.hako`
  - `try_handle(seg, regs, mname)`
    -> owns narrow `RuntimeDataBox.{get,set,has,push}` method dispatch for `mir_call_v1_handler`
  - `get_hh(recv_h, key_any)` / `set_hhh(recv_h, key_any, val_any)` / `has_hh(recv_h, key_any)` / `push_hh(recv_h, val_any)`
    -> `nyash.runtime_data.*` thin extern wrapper used by the same owner
- `map_core_box.hako`
  - `try_handle(seg, regs, mname)`
    -> owns adapter-on `MapBox.{set,get,has,size/len/length}` orchestration for `mir_call_v1_handler`
  - `size_i64(handle)` -> `nyash.map.size_h`
  - `norm_key_str(raw)` -> stable MapBox key normalization for adapter-on state
  - `record_set_state(...)` / `get_state_value(...)` / `has_state_value(...)`
    -> adapter-on MapBox state ownership for `mir_call_v1_handler`
- `string_core_box.hako`
  - `len_i64(handle)` -> `nyash.string.len_h`
  - `try_handle(seg, regs, mname)`
    -> owns adapter-on `StringBox.length/len/size` orchestration for `mir_call_v1_handler`

## Current proof lock

- source-contract smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
- standalone RuntimeDataBox e2e smoke:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
