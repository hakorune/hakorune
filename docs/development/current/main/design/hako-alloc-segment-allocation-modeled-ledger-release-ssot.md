---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-097A segment allocation modeled ledger release route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-594-MIMAP-097A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/
---

# Hako Alloc Segment Allocation Modeled Ledger Release

## Decision

`MIMAP-097A` adds a modeled release route on the scalar segment allocation
ledger.

The route marks exactly one live modeled allocation token as released:

```text
live modeled token
  -> live_flags[row] = 0
  -> live_count -= 1
  -> scalar release report
```

This is still not real segment free execution. It is a proof route for token
lifecycle in the scalar ledger.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
```

The owner may:

- find a modeled allocation token in the scalar ledger;
- reject invalid, missing, already-released, or unsupported release requests;
- mark one live row as no longer live;
- expose scalar release counters and report facts.

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
| `0` | released modeled ledger token |
| `1` | invalid token shape |
| `2` | modeled token not found |
| `3` | modeled token already released |
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
release_first=1,0,0,60018002,60,18,2,1,0,2,1
release_second=1,0,1,61019007,61,19,7,1,0,2,0
rejects=1,2,3,4,5,6,7,8,9,10,11,12
release_counts=14,2,12,1,1,1,1,1,1,1,1,1,1,1,61019007,0,1,0
```

## Stop Line

This row is ledger-only. It does not open:

```text
real free
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
