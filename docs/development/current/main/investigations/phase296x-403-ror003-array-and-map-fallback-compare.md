---
Status: Draft
Date: 2026-05-30
Scope: row403 ROR-003 array and map fallback compare
Related:
  - docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# ROR-003 Array And Map Fallback Compare

## Input

- `src/llvm_py/instructions/mir_call/collection_method_call.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_lower_array_collection_method_call` | array surface | `ArrayBox` | `296-346` | Keeps fail-safe arity defaults, then tries the exact direct-array lane, then the shared array symbol table. |
| `_lower_map_collection_method_call` | map surface | `MapBox` | `349-413` | Owns the map-only raw-kernel branches for `get/push/set/has/clear/delete`. |
| `select_array_collection_call_spec` | policy lookup | `ArrayBox` / runtime-data array route | `332-336` | Shared symbol-table policy for the array fallback branch. |

## Exact Callsites

- `if not arg_ids:` / `return zero` / `return recv_h` at `310-316`
- `direct_result = _lower_direct_array_nativedirect_call(...)` at `318-328`
- `spec = select_array_collection_call_spec(...)` at `332-336`
- `if method_name == "clear":` / `delete` / `get` / `push` / `set` / `has` at `362-411`

## Likely Miss Points

- Missing arguments stay fail-safe instead of creating a new route.
- The exact direct-array lane is tried before the shared array fallback, so the fallback branch is secondary by construction.
- `MapBox` stays separate and only claims the map raw-kernel cases.

## Verdict

The array and map branches are fallback surfaces, not primary owners. The direct-array lane remains the exact-only front door for the array surface.
