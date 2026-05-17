---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-104A segment allocation modeled ledger release span facts route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-602-MIMAP-104A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-SPAN-FACTS-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-proof/
---

# Hako Alloc Segment Allocation Modeled Ledger Release Span Facts

## Decision

`MIMAP-104A` extends the scalar modeled allocation ledger release report with
release span facts for a successful live-token release.

The contract is:

```text
live modeled token
  -> release token
  -> release report carries the original scalar allocation span
```

The span facts are read from the existing ledger row:

```text
old_page_used_at_allocation
page_capacity
request_blocks
new_page_used_at_allocation
remaining_blocks_at_allocation
modeled_block_start
modeled_block_end
released_blocks
release_span_present
```

This is not real segment free execution. It does not mutate a page free-list,
claim/unclaim bitmap state, use raw pointers, or return pages to OSVM.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
```

The owner may:

- enrich successful `releaseModeledToken(...)` reports with scalar span facts;
- keep rejection reports span-absent with `release_span_present == 0`;
- preserve MIMAP-097A release and MIMAP-100A released-token recycle behavior.

The owner must not:

- execute real segment allocation/free;
- mutate a free-list;
- mutate page state outside the modeled ledger;
- allocate arena backing;
- use raw pointer residence;
- perform segment-map pointer membership or lookup;
- claim or unclaim an atomic bitmap;
- call page-source / OSVM seams;
- schedule or spawn workers;
- activate providers, hooks, host allocator replacement, or
  `#[global_allocator]`;
- add backend `.inc` app/name matchers.

## Acceptance Shape

The proof app must expose at least:

```text
first_span=1,0,60018002,2,8,3,5,3,2,5,3
missing_span=0,2,0
recycled_span=1,1,60018002,2,8,3,5,3,2,5,3
counts=2,2,2,0,3,2,1,1
inactive=0,0,0,0,0,0,0,0,0
summary=ok
```

## Stop Line

This row is ledger-only. It does not open:

```text
real segment allocation/free
free-list mutation
page state mutation outside the modeled ledger
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
