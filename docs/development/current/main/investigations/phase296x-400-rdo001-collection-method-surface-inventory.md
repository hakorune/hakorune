---
Status: Draft
Date: 2026-05-30
Scope: row400 RDO-001 collection_method_call.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/utils/resolver_helpers.py
---

# RDO-001 Collection-Method Surface Inventory

## Input

- `src/llvm_py/instructions/mir_call/collection_method_call.py`

## Route Table

| function | route family | line_hint | note |
| --- | --- | --- | --- |
| `lower_collection_method_call` | dispatcher | `416-456` | Routes `RuntimeDataBox` first, then `ArrayBox`, then `MapBox`. |
| `_lower_array_collection_method_call` | array surface | `296-346` | Keeps fail-safe arity defaults, then tries the direct-array lane, then the canonical array symbol table. |
| `_lower_direct_array_nativedirect_call` | exact-lane direct-array surface | `227-252` | Active only when the exact env, selected method, receiver fact, and direct-array origin all line up. |
| `_lower_map_collection_method_call` | map surface | `349-413` | Map-only raw kernel routes for `get/push/set/has/clear/delete`. |
| `select_array_collection_call_spec` | policy lookup | imported | Shares the `nyash.array.slot_*` symbol table with `RuntimeDataBox(array-specialized)`. |

## Exact Callsites

- `runtime_result = lower_runtime_data_method_call(...)` at `429-440`
- `if str(box_name or "") == "ArrayBox":` at `444-454`
- `direct_result = _lower_direct_array_nativedirect_call(...)` at `318-328`
- `spec = select_array_collection_call_spec(...)` at `332-336`
- `if method_name == "get":` / `push` / `set` / `has` / `clear` / `delete` branches at `362-411`

## Likely Miss Points

- `RuntimeDataBox` routing preempts the collection split whenever `lower_runtime_data_method_call(...)` returns a spec.
- The direct-array lane is gated by `HAKO_ARRAY_SLOT_STORE == "direct_array_i64_exact"`, the exact selected method name, a receiver VID, and `is_arrayrepr_direct_i64(...)`.
- Missing arguments keep the existing fail-safe shape (`0` or `recv_h`) rather than opening a new route.
- Non-`i64` ArrayBox keys and unsupported exact-lane facts fall back to the canonical array symbol table or the `RuntimeDataBox` facade.
- `MapBox` stays a separate raw-kernel surface; it does not claim `ArrayBox` or direct-array ownership.

## Verdict

`collection_method_call.py` is a route-order dispatcher with a narrow direct-array branch. The direct-array lane is exact-lane only; it is not the general collection route.
