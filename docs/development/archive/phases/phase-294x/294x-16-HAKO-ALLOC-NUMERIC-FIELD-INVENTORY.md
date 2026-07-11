---
Status: Landed
Date: 2026-05-12
Scope: hako_alloc numeric stored field inventory before usize migration.
Related:
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-16 Hako Alloc Numeric Field Inventory

## Decision

Before migrating any live `hako_alloc` stored field to `usize`, every current
numeric stored field is classified in one place:

- signed sentinel;
- signed delta;
- count;
- size;
- capacity;
- index;
- byte length.

The SSOT is:

```text
lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
```

## Result

Current stored numeric field count:

```text
36
```

Only one stored field currently carries a negative sentinel:

```text
HakoAllocPageQueue.direct_page_index: i64 = -1
```

The inventory also records non-stored `-1` seams such as rejected block ids and
not-found direct-page indexes, because those affect the next sentinel split
row.

## Stop Line

This row does not migrate fields, change `.hako` runtime behavior, add new
`usize` lowering, or change allocator proofs. It only fixes the field map that
294x-17 and 294x-18 must consume.

## Verification

```bash
rg -n '^\s+[A-Za-z_][A-Za-z0-9_]*:\s*i64(?:\s*=\s*[-0-9]+)?' lang/src/hako_alloc
bash tools/checks/current_state_pointer_guard.sh
```
