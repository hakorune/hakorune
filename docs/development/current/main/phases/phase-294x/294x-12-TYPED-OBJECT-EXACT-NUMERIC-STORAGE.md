---
Status: Complete
Date: 2026-05-12
Scope: typed-object layout storage names for exact numeric declared fields.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/language/types.md
---

# 294x-12 Typed-Object Exact Numeric Storage

## Decision

Typed-object layout plans must distinguish exact numeric declared field storage
from legacy `i64` storage in the backend-readable plan surface.

This row makes `storage: "usize"` visible for a field declared as `usize`
instead of collapsing it to `storage: "i64"`. Runtime values and current field
get/set value types still use the dynamic `Integer(i64)` lane until later
backend/storage rows add exact native slots.

## Scope

This row owns:

- exact numeric storage variants in `TypedObjectFieldStorage`;
- declared-type storage inference for `i8`/`i16`/`i32`/`isize` and unsigned
  fixed-width/pointer-width names;
- MIR JSON typed-object plan emission preserving those storage names;
- integer-lane compatibility helpers so existing VM/reference behavior keeps
  using `MirType::Integer`.

## Stop Line

This row does not add:

- new VM value variants;
- native unsigned field storage;
- backend lowering for exact native slot widths;
- hako_alloc field migration;
- silent conversion of sentinel-bearing fields to unsigned storage.

The plan surface is now honest about declared exact numeric storage, while
execution remains on `Integer(i64)` and later rows must either lower exact slots
or fail fast.

