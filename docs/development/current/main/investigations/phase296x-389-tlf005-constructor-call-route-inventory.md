---
Status: Draft
Date: 2026-05-30
Scope: row389 TLF-005 constructor_call.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-389-TYPED-OBJECT-LEGACY-FIELD-HELPER-OWNER-INVENTORY.md
  - src/llvm_py/instructions/mir_call/constructor_call.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/utils/resolver_helpers.py
---

# TLF-005 Constructor Call Route Inventory

## Input

- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/mir_call/constructor_call.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_direct_array_i64_constructor_enabled` | gate | `HAKO_ARRAY_SLOT_STORE == "direct_array_i64_exact"` | `18-19` | Exact-lane selector; if this mismatches, `ArrayRepr::DirectI64` is never produced here. |
| `_mark_direct_array_i64_origin` | producer | `ArrayRepr::DirectI64` | `22-31` | Writes the direct-array origin fact into the resolver facts store. |
| `lower_constructor_call` ArrayBox branch | producer | `ArrayRepr::DirectI64` | `89-100` | Selects `nyash.array.direct_i64.birth_h` for `ArrayBox` when the exact-lane env gate is open, then marks the direct-array origin. |

## Exact Callsites

- `DIRECT_ARRAY_I64_BIRTH_SYMBOL = "nyash.array.direct_i64.birth_h"` at `14`
- `PUBLIC_ARRAY_BIRTH_SYMBOL = "nyash.array.birth_h"` at `15`
- `direct_array_birth = _direct_array_i64_constructor_enabled()` at `92`
- `callee = _declare(DIRECT_ARRAY_I64_BIRTH_SYMBOL if direct_array_birth else PUBLIC_ARRAY_BIRTH_SYMBOL, ...)` at `93-96`
- `result = builder.call(callee, [], name="unified_arr_new")` at `98`
- `_mark_direct_array_i64_origin(resolver, dst_vid)` at `100`
- `mark_arrayrepr_direct_i64(resolver, int(dst_vid))` at `29`

## Likely Miss Points

- `selected-method-only gate`
- `env mismatch`

`constructor_call.py` does not consume `ArrayRepr::DirectI64`; it only produces the fact for exact-lane `ArrayBox` births, so the file reads as a producer with narrow gating risk rather than a consumer.

## Verdict

`constructor_call.py` looks like a producer, with the only likely miss points being the exact-lane env gate and the exact birth-symbol split between public ArrayBox and DirectArray.
