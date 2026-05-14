# 293x-315 ARRAY-002A typed Array method contract

Status: complete

## Decision

Decision: accepted.

`Array<T>` locals now carry a Stage1 method-surface contract for the canonical
ordinary typed collection methods `push`, `get`, `set`, and `length`.
Diagnostics are fail-fast and tagged with `[array/method-contract]`.

## Scope

- Track local declarations with `Array<T>` typed context in Program JSON v0
  lowering.
- Validate typed local receiver method names.
- Validate canonical method arities:
  - `push(value)`
  - `get(index)`
  - `set(index, value)`
  - `length()`
- Keep existing generic JSON v0 `Method` lowering shape.

## Non-goals

- No element type checking. `ARRAY-002B` owns that row.
- No inference for untyped arrays. `ARRAY-002C` owns that row.
- No PackedArray fallback or backend route proof. `ARRAY-002D` owns that row.
- No canonical aliases such as `len` or `size`.

## Guard

- `tools/checks/k2_wide_array_typed_method_contract_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_array_typed_method_contract_guard.sh` passed locally.
