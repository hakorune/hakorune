---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-091A segment allocation modeled consume route.
Related:
  - docs/development/current/main/phases/phase-293x/293x-588-MIMAP-091A-SEGMENT-ALLOCATION-MODELED-CONSUME-ROUTE.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako
  - apps/hako-alloc-segment-allocation-modeled-consume-proof/
---

# Hako Alloc Segment Allocation Modeled Consume

## Decision

`MIMAP-091A` adds a narrow modeled scalar consume route after segment allocation
readiness has accepted a known page.

The route is deliberately not a real segment allocation/free path. It consumes
scalar facts and reports the modeled post-allocation page usage:

```text
readiness accepted
  + old page_used
  + request_blocks
  -> new_page_used
  -> remaining_blocks
  -> modeled block-start / token
```

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako
```

The owner may:

- validate a scalar readiness result;
- compute `new_page_used = old_page_used + request_blocks`;
- compute `remaining_blocks = page_capacity - new_page_used`;
- expose a stable modeled block-start scalar;
- expose a stable modeled allocation token for proof output;
- count accepted and rejected modeled consumes.

The owner must not:

- execute real segment allocation/free;
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
| `0` | accepted modeled consume |
| `1` | upstream readiness rejected |
| `2` | invalid scalar shape |
| `3` | capacity changed or request no longer fits |
| `4` | real segment allocation/free execution requested |
| `5` | raw pointer residence requested |
| `6` | segment-map lookup requested |
| `7` | arena backing allocation requested |
| `8` | atomic bitmap execution requested |
| `9` | OSVM/page-source execution requested |
| `10` | thread/worker execution requested |
| `11` | provider activation requested |
| `12` | backend matcher requested |

## Acceptance Shape

The proof app must expose at least:

```text
success=1,0,60,18,2,3,5,3,2,60018002
edge=1,0,61,19,7,1,8,0,7,61019007
rejects=1,2,3,4,5,6,7,8,9,10,11,12
inactive=0,0,0,0,0,0,0,0,0
```

## Stop Line

This row is still scalar model execution. It does not open:

```text
arena residence
raw pointer residence
segment-map lookup
atomic bitmap execution
OSVM/page-source execution
thread scheduling
provider activation
host allocator replacement
backend matchers
```
