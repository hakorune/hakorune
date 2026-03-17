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
  - `record_push_state(...)` / `record_set_state(...)` / `get_state_value(...)`
    -> adapter-on ArrayBox size/value-state ownership for `mir_call_v1_handler`
- `map_core_box.hako`
  - `size_i64(handle)` -> `nyash.map.size_h`
  - `norm_key_str(raw)` -> stable MapBox key normalization for adapter-on state
  - `record_set_state(...)` / `get_state_value(...)` / `has_state_value(...)`
    -> adapter-on MapBox state ownership for `mir_call_v1_handler`
- `string_core_box.hako`
  - `len_i64(handle)` -> `nyash.string.len_h`
