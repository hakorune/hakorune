---
Status: Draft
Date: 2026-05-30
Scope: row400 RDO-002 boxcall_runtime_data.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - src/llvm_py/instructions/boxcall_runtime_data.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
---

# RDO-002 BoxCall Collection Surface Inventory

## Input

- `src/llvm_py/instructions/boxcall_runtime_data.py`

## Route Table

| function / branch | route family | line_hint | note |
| --- | --- | --- | --- |
| `try_lower_collection_boxcall` | collection dispatcher | `21-151` | Owns the ArrayBox/MapBox/RuntimeData-like collection route order for BoxCall lowering. |
| `size` branch | size facade | `58-70` | Prefers string, then array, then map, then `Any.length_h`. |
| `get` branch | array/map split | `72-86` | ArrayBox/arrayish receivers go through `select_array_collection_call_spec`; everything else goes to `nyash.map.slot_load_hh`. |
| `push` branch | array route | `88-95` | Delegates to `select_array_collection_call_spec` only. |
| `set` branch | array/map split | `97-114` | ArrayBox/arrayish receivers use `select_array_collection_call_spec`; others go to `nyash.map.slot_store_hhh`. |
| `has` branch | array/map split | `116-130` | ArrayBox/arrayish receivers use `select_array_collection_call_spec`; others go to `nyash.map.probe_hh`. |
| `clear` / `delete` branches | map-only route | `132-149` | These are `MapBox`-only routes and stay outside the ArrayBox surface. |

## Exact Callsites

- `known_box_name = get_box_type(resolver, box_vid)` at `59`, `73`, `98`, `117`, `133`, `141`
- `select_array_collection_call_spec(...)` at `75-81`, `89-95`, `100-106`, `119-125`
- `callee = declare(module, "nyash.string.len_h", ...)` at `61-62`
- `callee = declare(module, "nyash.array.slot_len_h", ...)` at `64-65`
- `callee = declare(module, "nyash.map.entry_count_i64", ...)` at `67-68`
- `callee = declare(module, "nyash.map.slot_load_hh", ...)` at `85-86`
- `callee = declare(module, "nyash.array.slot_append_hh", ...)` at `90-95`
- `callee = declare(module, "nyash.map.slot_store_hhh", ...)` at `113-114`
- `callee = declare(module, "nyash.map.probe_hh", ...)` at `129-130`
- `callee = declare(module, "nyash.map.clear_h", ...)` at `137-138`
- `callee = declare(module, "nyash.map.delete_hh", ...)` at `148-149`

## Likely Miss Points

- `receiver_is_arrayish` / `receiver_is_mapish` / `get_box_type` disagreement can move the route between ArrayBox, MapBox, and the generic facade.
- The size route is intentionally broader than the rest of the collection surface and will accept string, array, map, or `Any.length_h`.
- The array routes do not open a direct-array fast path here; they still depend on the shared array call spec selector.
- `clear` and `delete` remain map-only and return early when the receiver is not `MapBox` or mapish.

## Verdict

`boxcall_runtime_data.py` is a thin route-order wrapper. It keeps collection routing centralized and does not own the direct-array substrate.
