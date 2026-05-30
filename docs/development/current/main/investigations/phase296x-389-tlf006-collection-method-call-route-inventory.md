---
Status: Draft
Date: 2026-05-30
Scope: row389 TLF-006 collection_method_call.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-389-TYPED-OBJECT-LEGACY-FIELD-HELPER-OWNER-INVENTORY.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/utils/resolver_helpers.py
---

# TLF-006 Collection Method Call Route Inventory

## Input

- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call/collection_method_call.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_direct_array_nativedirect_selected` | gate | `ArrayRepr::DirectI64` lookup plus `HAKO_ARRAY_SLOT_STORE == "direct_array_i64_exact"` | `42-52` | Exact-lane selector; this file only consumes the fact when the selected-method gate and env gate both pass. |
| `_lower_direct_array_nativedirect_call` | consumer | `ArrayRepr::DirectI64` | `227-252` | DirectArray get/set fast path consumes the fact and lowers the hot i64 route directly. |
| `_lower_array_collection_method_call` | router | `ArrayRepr::DirectI64` fallback path | `296-346` | Direct path is tried first; then the canonical ArrayBox / RuntimeDataBox route is used if direct lowering is not selected. |
| `lower_collection_method_call` | router | `ArrayRepr::DirectI64` input to array dispatch | `388-422` | Dispatches ArrayBox through runtime_data first, then the direct array path, then map routing. |

## Exact Callsites

- `from utils.resolver_helpers import is_arrayrepr_direct_i64` at `17`
- `if os.environ.get("HAKO_ARRAY_SLOT_STORE") != "direct_array_i64_exact":` at `43`
- `return is_arrayrepr_direct_i64(resolver, int(receiver_vid))` at `50`
- `direct_result = _lower_direct_array_nativedirect_call(...)` at `318-326`
- `if direct_result is not None: return direct_result` at `327-328`
- `spec = select_array_collection_call_spec(...)` at `332-336`
- `return _lower_call_spec(...)` at `339-346`

## Likely Miss Points

- `selected-method-only gate`
- `env mismatch`
- `RuntimeDataBox`
- `copy/PHI carrier`

`collection_method_call.py` does consume `ArrayRepr::DirectI64`, but only through the narrow selected-method route, and the fallback still routes through the canonical runtime ArrayBox / RuntimeDataBox surface.

## Verdict

`collection_method_call.py` looks like a consumer, with the main miss points being the selected-method gate, the env gate, and the fallback RuntimeDataBox route.
