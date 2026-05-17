---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-115A segment allocation modeled local-free page-model apply route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md
  - docs/development/current/main/design/mimalloc-page-free-list-pilot-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-614-MIMAP-115A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-PAGE-MODEL-APPLY-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako
  - apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/
---

# Hako Alloc Segment Allocation Modeled Local-Free Page Apply

## Decision

`MIMAP-115A` applies a successful scalar local-free apply-plan report to an
explicit `HakoAllocPageModel`.

The contract is:

```text
successful local-free apply-plan report
  + explicit HakoAllocPageModel
  -> validate page id / block span / live block shape
  -> call HakoAllocPageModel.releaseLocal(block_id) for every block in span
```

This row opens only the existing page-local mutation seam owned by
`HakoAllocPageModel`. It does not introduce a second page-state owner.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako
```

The owner may:

- consume successful `HakoAllocSegmentAllocationModeledLocalFreeApplyPlanReport`
  values;
- accept an explicitly supplied `HakoAllocPageModel`;
- validate page id, block span, and live-block shape;
- call `HakoAllocPageModel.releaseLocal(block_id)` for each span block;
- record deterministic `(apply-plan row index, token)` applied rows;
- expose scalar page used/local-free/free counts before and after;
- reject invalid, source-rejected, duplicate, page-id mismatch, page-block, or
  unsupported requests.

The owner must not:

- execute real segment allocation/free beyond the existing page-local model;
- mutate page arrays directly;
- allocate arena backing;
- use raw pointer residence;
- perform segment-map pointer membership or lookup;
- claim or unclaim an atomic bitmap;
- call page-source / OSVM seams;
- schedule or spawn workers;
- activate providers, hooks, host allocator replacement, or
  `#[global_allocator]`;
- add backend `.inc` app/name matchers.

## Reason Codes

| Code | Meaning |
| ---: | --- |
| `0` | applied local-free span to page model |
| `1` | invalid scalar shape |
| `2` | source local-free apply-plan report rejected or absent |
| `3` | duplicate `(apply-plan row index, token)` page apply |
| `4` | explicit page id does not match plan page id |
| `5` | span block is not live or `releaseLocal` rejected |
| `6` | real segment allocation/free execution requested |
| `7` | raw pointer residence requested |
| `8` | segment-map lookup requested |
| `9` | arena backing allocation requested |
| `10` | atomic bitmap execution requested |
| `11` | OSVM/page-source execution requested |
| `12` | thread/worker execution requested |
| `13` | provider activation requested |
| `14` | backend matcher requested |

## Acceptance Shape

The proof app must expose at least:

```text
first=1,0,0,-1,0,60018002,60,18,2,5,3,3,6,3,0,3
missing=0,2,-1,1
duplicate=0,3,0,1
wrong_page=0,4,1
unsupported=0,6,1
recycled=1,0,1,-1,1,60018002,60,18,2,5,3,3,6,3,0,3
counts=6,2,2,4,0,1,1,1,0,1
page=3,2,3,5
inactive=0,0,0,0,0,0,0,0,0,0
summary=ok
```

## Stop Line

This row remains a page-model apply route. It does not open:

```text
real segment allocation/free beyond the existing page-local model
direct page array mutation
raw pointer residence
segment-map lookup
arena backing allocation
atomic bitmap execution
OSVM/page-source execution
thread scheduling
provider activation
host allocator replacement
backend matchers
```
