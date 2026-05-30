---
Status: Draft
Date: 2026-05-30
Scope: row403 ROR-002 direct-array lane compare
Related:
  - docs/development/current/main/phases/phase-296x/296x-403-COLLECTION-METHOD-ROUTE-ORDER-INVENTORY.md
  - src/llvm_py/instructions/mir_call/collection_method_call.py
  - src/llvm_py/utils/resolver_helpers.py
---

# ROR-002 Direct Array Lane Compare

## Input

- `src/llvm_py/instructions/mir_call/collection_method_call.py`
- `src/llvm_py/utils/resolver_helpers.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_direct_array_nativedirect_selected` | gate | `ArrayRepr::DirectI64` + env gate | `42-52` | Exact-lane selector; it only passes when the selected-method gate, env gate, resolver fact, and receiver VID all line up. |
| `_lower_direct_array_nativedirect_call` | consumer | `ArrayRepr::DirectI64` | `227-252` | DirectArray `get/set` fast path consumes the fact and lowers the hot i64 route directly. |
| `is_arrayrepr_direct_i64` | fact query | `ArrayRepr::DirectI64` | `191-192` | Explicit fact lookup that the direct-array gate relies on. |
| `mark_arrayrepr_direct_i64` | producer | `ArrayRepr::DirectI64` | `174-175` | Exact-lane origin marker used by constructor / fact-production code paths outside this file. |

## Exact Callsites

- `if os.environ.get("HAKO_ARRAY_SLOT_STORE") != "direct_array_i64_exact":` at `43-44`
- `if _current_function_name(builder) != DIRECT_ARRAY_NATIVEDIRECT_SELECTED_METHOD:` at `45-46`
- `return is_arrayrepr_direct_i64(resolver, int(receiver_vid))` at `50`
- `if not _direct_array_nativedirect_selected(builder, resolver, receiver_vid):` at `237-238`
- `if method_name == "get":` / `if method_name == "set":` at `241-251`
- `return mark_arrayrepr_direct_i64(resolver, vid)` at `174-175` in `resolver_helpers.py`

## Likely Miss Points

- `selected-method-only gate`
- `env mismatch`
- `copy/PHI carrier`

## Verdict

`collection_method_call.py` is a consumer with an exact-only direct-array lane, not a general direct-array producer.
