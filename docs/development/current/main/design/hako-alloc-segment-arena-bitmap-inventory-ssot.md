---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-079A segment / arena / bitmap boundary inventory.
Related:
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-566-MIMAP-079A-SEGMENT-ARENA-BITMAP-BOUNDARY-INVENTORY.md
  - lang/src/hako_alloc/memory/segment_arena_bitmap_inventory_box.hako
  - apps/hako-alloc-segment-arena-bitmap-inventory-proof/
---

# Hako Alloc Segment Arena Bitmap Inventory SSOT

## Decision

`MIMAP-079A` adds a scalar allocator-owned inventory for segment / arena /
bitmap boundaries.

The route names tiny proof-only segment/arena/mask facts and explicit blocked
reasons for requirements that must not be hidden behind scalar fallback:

```text
raw pointer residence
atomic bitmap execution
OSVM execution
provider activation
invalid segment/arena/mask shape
```

It does not implement segment allocation, arena memory routing, bitmap claim,
page-source behavior, provider activation, or host allocator replacement.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_bitmap_inventory_box.hako
```

Responsibilities:

```text
classify tiny scalar segment/arena/bitmap facts
report committed/purge scalar mask counts for proof-only shapes
reject invalid and unsupported requirements with stable reasons
report inactive raw pointer / atomic bitmap / OSVM / provider flags
```

Non-responsibilities:

```text
segment allocation or free
arena memory allocation
raw pointer residence
atomic bitmap claim/unclaim
page-source or OSVM execution
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | scalar proof-only segment/arena/bitmap facts accepted |
| `1` | invalid segment, arena, slice, commit, or purge shape |
| `2` | raw pointer residence is required |
| `3` | atomic bitmap execution is required |
| `4` | OSVM execution is required |
| `5` | provider activation is requested |

## Proof Surface

```text
apps/hako-alloc-segment-arena-bitmap-inventory-proof/
tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_guard.sh
```

Required inactive facts:

```text
would_use_raw_pointer = 0
would_execute_atomic_bitmap = 0
would_call_osvm = 0
would_activate_provider = 0
would_replace_process_allocator = 0
would_add_backend_matcher = 0
```

## Stop Lines

No part of `MIMAP-079A` may add:

```text
allocation/free behavior
real thread scheduling
worker spawning
source-level concurrency semantics
raw pointer residence
atomic bitmap execution
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```
