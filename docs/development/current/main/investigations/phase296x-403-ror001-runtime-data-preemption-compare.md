---
Status: Draft
Date: 2026-05-30
Scope: row403 ROR-001 runtime-data preemption compare
Related:
  - docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
---

# ROR-001 Runtime-Data Preemption Compare

## Input

- `src/llvm_py/instructions/mir_call/collection_method_call.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `lower_collection_method_call` | dispatcher | `RuntimeDataBox` preemption | `429-456` | `lower_runtime_data_method_call(...)` runs before the ArrayBox / MapBox split, so the shared route order still gives runtime-data first chance. |
| `_lower_array_collection_method_call` | array surface | `ArrayBox` | `296-346` | Runs only after the runtime-data preemption misses; then it tries the exact direct-array lane and the canonical array symbol table. |
| `_lower_map_collection_method_call` | map surface | `MapBox` | `349-413` | Runs only after the ArrayBox branch misses; it owns the map raw-kernel fallback path. |

## Exact Callsites

- `runtime_result = lower_runtime_data_method_call(...)` at `429-440`
- `if runtime_result is not None:` / `return runtime_result` at `441-442`
- `if str(box_name or "") == "ArrayBox":` at `444-454`
- `return _lower_map_collection_method_call(...)` at `456-464`

## Likely Miss Points

- `RuntimeDataBox` preemption stays thin but can still swallow the collection split if the route policy returns a spec too early.
- Missing-argument defaults stay fail-safe (`0` or `recv_h`) rather than opening a new route.
- `ArrayBox` and `MapBox` remain separate late branches; they are not interchangeable with the runtime-data preemption.

## Verdict

`lower_collection_method_call` is a route-order dispatcher with runtime-data preemption first, then ArrayBox, then MapBox. The preemption is a guard, not the durable owner.
