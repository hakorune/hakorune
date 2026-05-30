---
Status: Draft
Date: 2026-05-30
Scope: row400 RDO-004 legacy bridge surface inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - src/llvm_py/instructions/mir_call_legacy.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
---

# RDO-004 Legacy Bridge Surface Inventory

## Input

- `src/llvm_py/instructions/mir_call_legacy.py`

## Route Table

| function / branch | route family | line_hint | note |
| --- | --- | --- | --- |
| `lower_legacy_mir_call` RuntimeDataBox branch | legacy compatibility bridge | `315-325` | Sends only `RuntimeDataBox.getField/setField` through the shared field dispatcher. |

## Exact Callsites

- `elif box_name == "RuntimeDataBox" and method in {"getField", "setField"}:` at `315-325`
- `lower_runtime_data_field_call(...)` at `316-325`

## Likely Miss Points

- Only `getField` and `setField` are bridged here; other RuntimeDataBox methods do not claim this path.
- The bridge is thin and returns `None` when the shared field dispatcher does not own the method.
- This surface is compatibility-only; it does not introduce any direct-array or new collection policy.

## Verdict

`mir_call_legacy.py` keeps the legacy RuntimeDataBox bridge intact, but the bridge is only a compatibility consumer of the shared field dispatcher.
