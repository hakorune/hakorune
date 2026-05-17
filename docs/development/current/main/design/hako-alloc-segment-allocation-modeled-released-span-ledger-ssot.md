---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-107A segment allocation modeled released-span ledger route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-span-facts-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-606-MIMAP-107A-SEGMENT-ALLOCATION-MODELED-RELEASED-SPAN-LEDGER-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-released-span-ledger-proof/
---

# Hako Alloc Segment Allocation Modeled Released-Span Ledger

## Decision

`MIMAP-107A` adds a scalar released-span ledger downstream of `MIMAP-104A`.

The contract is:

```text
successful modeled release report
  -> release_span_present == 1
  -> record token / segment / page / block span in a separate ledger
```

This is a ledger-only bridge toward future local-free/free-list rows. It does
not execute real segment free and does not mutate a page free-list.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako
```

The owner may:

- consume successful `HakoAllocSegmentAllocationModeledLedgerReleaseReport`
  values;
- validate scalar released span shape;
- record deterministic `(release row index, token)` rows;
- expose scalar released-span read methods and counters;
- reject invalid, span-missing, duplicate, or unsupported requests.

The owner must not:

- execute real segment allocation/free;
- mutate a free-list;
- mutate page state outside the new scalar ledger;
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
| `0` | recorded released span |
| `1` | invalid scalar shape |
| `2` | release report rejected or span absent |
| `3` | duplicate `(release row index, token)` released span |
| `4` | real segment allocation/free execution requested |
| `5` | raw pointer residence requested |
| `6` | segment-map lookup requested |
| `7` | arena backing allocation requested |
| `8` | atomic bitmap execution requested |
| `9` | OSVM/page-source execution requested |
| `10` | thread/worker execution requested |
| `11` | provider activation requested |
| `12` | backend matcher requested |
| `13` | free-list mutation requested |

## Acceptance Shape

The proof app must expose at least:

```text
first=1,0,0,-1,60018002,60,18,2,5,3,1,1
missing=0,2,-1,1
duplicate=0,3,0,1
recycled=1,0,1,-1,60018002,60,18,2,5,3,2,2
unsupported=0,4,1
counts=5,2,2,3,0,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0
summary=ok
```

## Stop Line

This row remains scalar ledger execution. It does not open:

```text
real segment allocation/free
free-list mutation
page state mutation outside the released-span ledger
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
