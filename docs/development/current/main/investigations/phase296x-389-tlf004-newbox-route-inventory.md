---
Status: Draft
Date: 2026-05-30
Scope: row389 TLF-004 newbox.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-389-TYPED-OBJECT-LEGACY-FIELD-HELPER-OWNER-INVENTORY.md
  - src/llvm_py/instructions/newbox.py
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/utils/resolver_helpers.py
---

# TLF-004 NewBox Route Inventory

## Input

- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/instructions/newbox.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_direct_array_i64_constructor_enabled` | gate | `HAKO_ARRAY_SLOT_STORE == "direct_array_i64_exact"` | `18-19` | Exact-lane selector; if this mismatches, `ArrayRepr::DirectI64` is never produced here. |
| `_mark_direct_array_i64_origin` | producer | `ArrayRepr::DirectI64` | `52-60` | Writes both `resolver.direct_array_i64_ids` and the explicit arrayrepr fact store. |
| `lower_newbox` ArrayBox branch | producer | `ArrayRepr::DirectI64` | `172-191` | Selects `nyash.array.direct_i64.birth_h` for `ArrayBox` when the exact-lane env gate is open, then marks the direct-array origin. |

## Exact Callsites

- `DIRECT_ARRAY_I64_BIRTH_SYMBOL = "nyash.array.direct_i64.birth_h"` at `14`
- `PUBLIC_ARRAY_BIRTH_SYMBOL = "nyash.array.birth_h"` at `15`
- `direct_array_birth = box_type == "ArrayBox" and _direct_array_i64_constructor_enabled()` at `174`
- `birth_name = DIRECT_ARRAY_I64_BIRTH_SYMBOL if direct_array_birth else PUBLIC_ARRAY_BIRTH_SYMBOL` at `175-179`
- `handle = builder.call(birth, [], name=f"birth_{box_type}")` at `187`
- `_mark_direct_array_i64_origin()` at `191`
- `mark_arrayrepr_direct_i64(resolver, int(dst_vid))` at `59`

## Likely Miss Points

- `selected-method-only gate`
- `env mismatch`

`newbox.py` does not consume `ArrayRepr::DirectI64`; it only produces the fact for exact-lane `ArrayBox` births, so the file reads as a producer with narrow gating risk rather than a consumer.

## Verdict

`newbox.py` looks like a producer, with the only likely miss points being the exact-lane env gate and the earlier exact-object/local-user-box short-circuits.
