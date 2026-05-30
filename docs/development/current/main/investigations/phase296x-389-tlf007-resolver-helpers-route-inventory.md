---
Status: Draft
Date: 2026-05-30
Scope: row389 TLF-007 resolver_helpers.py route inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-389-TYPED-OBJECT-LEGACY-FIELD-HELPER-OWNER-INVENTORY.md
  - src/llvm_py/utils/resolver_helpers.py
  - src/llvm_py/instructions/newbox.py
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# TLF-007 Resolver Helpers Route Inventory

## Input

- `src/llvm_py/instructions/field_access.py`
- `src/llvm_py/utils/resolver_helpers.py`

## Route Table

| function | direction | fact | line_hint | note |
| --- | --- | --- | --- | --- |
| `_arrayrepr_fact_store` | bridge | `arrayrepr_facts` mutable store | `144-156` | Creates the shared fact carrier that producer and consumer routes depend on. |
| `record_arrayrepr_fact` | producer | explicit arrayrepr fact string | `159-170` | Writes the fact into the shared store. |
| `mark_arrayrepr_direct_i64` | producer | `ArrayRepr::DirectI64` | `173-175` | Canonical producer helper for the explicit direct-i64 fact. |
| `get_arrayrepr_fact` | consumer | explicit arrayrepr fact string | `178-187` | Reads back the fact for downstream checks. |
| `is_arrayrepr_direct_i64` | consumer | `ArrayRepr::DirectI64` | `190-192` | Predicate used by downstream consumers such as `collection_method_call.py`. |

## Exact Callsites

- `facts = getattr(resolver, "arrayrepr_facts", None)` at `149`
- `facts = {}` / `setattr(resolver, "arrayrepr_facts", facts)` at `152-153`
- `facts[int(vid)] = fact` at `167`
- `return record_arrayrepr_fact(resolver, vid, "ArrayRepr::DirectI64")` at `175`
- `fact = facts.get(int(vid))` at `184`
- `return get_arrayrepr_fact(resolver, vid) == "ArrayRepr::DirectI64"` at `192`

## Likely Miss Points

- `copy/PHI carrier`
- `selected-method-only gate`
- `env mismatch`

`resolver_helpers.py` is not a lowering site by itself; it is the shared carrier that must be kept consistent so producers can mark the fact and consumers can read it back.

## Verdict

`resolver_helpers.py` looks like a miss point, because the fact store is only a carrier unless the producer and consumer routes keep propagating the explicit `ArrayRepr::DirectI64` mark.
