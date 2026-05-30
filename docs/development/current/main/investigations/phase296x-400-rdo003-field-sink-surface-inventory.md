---
Status: Draft
Date: 2026-05-30
Scope: row400 RDO-003 field sink surface inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - src/llvm_py/instructions/mir_call/method_call.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
---

# RDO-003 Field Sink Surface Inventory

## Input

- `src/llvm_py/instructions/mir_call/method_call.py`
- `src/llvm_py/instructions/field_access.py`

## Route Table

| function / branch | route family | line_hint | note |
| --- | --- | --- | --- |
| `lower_method_call` RuntimeDataBox branch | compatibility sink | `288-298` in `method_call.py` | Routes `RuntimeDataBox.getField/setField` to the shared field dispatcher. |
| `lower_field_get` local-user-box branch | local field sink | `1078-1099` in `field_access.py` | Local user-box and exact-object routes short-circuit before RuntimeDataBox. |
| `lower_field_get` fallback branch | RuntimeDataBox sink | `1084-1098` in `field_access.py` | Boxes the field name, calls `RuntimeDataBox.getField`, and tags the result as a handle. |
| `lower_field_set` local-user-box branch | local field sink | `1116-1127` in `field_access.py` | Local user-box routes short-circuit before RuntimeDataBox. |
| `lower_field_set` fallback branch | RuntimeDataBox sink | `1217-1229` in `field_access.py` | Boxes the field name and value, then calls `RuntimeDataBox.setField`. |

## Exact Callsites

- `elif box_name == "RuntimeDataBox" and method in {"getField", "setField"}:` at `288-298` in `method_call.py`
- `exact_field_plan = _exact_field_plan_for_receiver(...)` at `1128-1143` in `field_access.py`
- `result = lower_runtime_data_field_call(... method="getField" ...)` at `1084-1099`
- `result = lower_runtime_data_field_call(... method="setField" ...)` at `1217-1229`
- `_boxed_field_key(builder, module, field_name)` at `1083` and `1212`

## Likely Miss Points

- Local user-box lowering and exact-object plans can absorb the field access before RuntimeDataBox ever sees it.
- Typed float/bool/integer field gating can also intercept the route before the sink fallback.
- The field name is always boxed into the map ABI before `lower_runtime_data_field_call(...)`, so the fallback surface is string-keyed rather than direct-slot keyed.
- A `None` return from `lower_runtime_data_field_call(...)` still falls back to `0`, so the sink surface is intentionally conservative.

## Verdict

`RuntimeDataBox.getField/setField` is the last fallback sink after local, exact-object, and typed field routes. It is a compatibility surface, not a direct-array surface.
