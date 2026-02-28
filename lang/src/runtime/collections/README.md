# lang/src/runtime/collections — Ring1 Collection Runtime Core

Scope: VM low-level collection runtime wrappers for cutover phases.

## Responsibility

- Provide thin `.hako` wrappers for collection ABI vocabularies used by VM core.
- Keep std-layer helpers (`apps/std/*`) out of VM low-level execution path.
- Delegate storage/primitive operations to ABI symbols (`nyash.array.*`, etc.) without adding policy logic.

## Current modules

- `array_core_box.hako`
  - `get_i64(handle, idx)` -> `nyash.array.get_hi`
  - `set_i64(handle, idx, value)` -> `nyash.array.set_hii`
- `string_core_box.hako`
  - `len_i64(handle)` -> `nyash.string.len_h`
