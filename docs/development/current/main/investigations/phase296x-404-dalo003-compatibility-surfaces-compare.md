---
Status: Draft
Date: 2026-05-30
Scope: row404 DALO-003 compatibility surfaces compare
Related:
  - docs/development/current/main/phases/phase-296x/296x-404-COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION.md
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call_legacy.py
---

# DALO-003 Compatibility Surfaces Compare

## Input

- `src/llvm_py/instructions/boxcall_runtime_data.py`
- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call_legacy.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `try_lower_collection_boxcall` | compatibility surface | `ArrayBox` / `MapBox` / `RuntimeDataBox` | `21-151` | BoxCall compatibility surface that still uses `select_array_collection_call_spec(...)` for ArrayBox and keeps runtime-data as a separate surface. |
| `lower_field_get` | compatibility surface | exact typed fields / `RuntimeDataBox` fallback | `971-1099` | Tries local user-box fields, exact field plans, then typed field helper routes, then the runtime-data field fallback. |
| `lower_field_set` | compatibility surface | exact typed fields / `RuntimeDataBox` fallback | `1102-1229` | Mirrors `lower_field_get` with the same exact-plan and compatibility fallback layering. |
| `lower_collection_method_call` in `mir_call_legacy.py` | legacy bridge | shared collection route | `279-289` | Thin bridge from the legacy call entrypoint into the shared collection route order. |

## Exact Callsites

- `select_array_collection_call_spec(...)` at `72-106` in `boxcall_runtime_data.py`
- `result = lower_collection_method_call(...)` at `279-289` in `mir_call_legacy.py`
- `exact_field_plan = _exact_field_plan_for_receiver(...)` at `998-1013`
- `result = lower_runtime_data_field_call(...)` at `1084-1093`
- `result = lower_runtime_data_field_call(...)` at `1217-1226`

## Likely Miss Points

- The BoxCall compatibility surface is secondary; it only stays in play when the shared collection route does not resolve earlier.
- `field_access.py` can absorb exact typed-field plans before it falls back to runtime-data field helpers, so it remains a compatibility seam rather than a durable owner.
- `mir_call_legacy.py` is a thin bridge, not a separate semantic owner.

## Verdict

The compatibility surfaces stay secondary. They are bridging layers, not the primary route-order owner.
