---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-100A segment allocation modeled ledger released-token recycle route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-597-MIMAP-100A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASED-TOKEN-RECYCLE-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-proof/
---

# Hako Alloc Segment Allocation Modeled Ledger Released-Token Recycle

## Decision

`MIMAP-100A` proves that the scalar modeled allocation ledger can recycle a
released token without opening real segment allocation/free execution.

The contract is:

```text
record token as live
  -> live duplicate is rejected
  -> release token
  -> same scalar token may be recorded again as the new live row
  -> simultaneous live duplicate is still rejected
```

This models allocator block reuse at the scalar ledger level only. It is not a
free-list, segment-map, raw pointer, or atomic bitmap implementation.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
```

The owner already exposes the needed behavior:

- `findIndex(token)` returns only live rows;
- `findAnyIndex(token)` can still identify historical rows;
- `recordModeledConsume(...)` rejects live duplicates;
- `releaseModeledToken(...)` marks a live row released and uses historical rows
  only for already-released diagnostics when no live row exists.

MIMAP-100A should avoid owner growth unless a tiny helper is needed to keep this
contract local and explicit.

## Acceptance Shape

The proof app must expose at least:

```text
first=1,0,0,-1,60018002,1,1
duplicate_live=0,4,0
release_first=1,0,0,1,0,0
after_release=-1,0,-1
recycled=1,0,1,-1,60018002,2,1
after_recycle=1,0,1,60018002
duplicate_after_recycle=0,4,1
release_recycled=1,0,1,1,0,0
counts=4,2,2,2,2,0,2,2,0,60018002,0,1
```

## Stop Line

This row is ledger-only. It does not open:

```text
real segment allocation/free
arena backing allocation
raw pointer residence
segment-map lookup
atomic bitmap execution
OSVM/page-source execution
thread scheduling
provider activation
host allocator replacement
backend matchers
```
