---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-109A segment allocation modeled local-free candidate ledger route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-608-MIMAP-109A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-CANDIDATE-LEDGER-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/
---

# Hako Alloc Segment Allocation Modeled Local-Free Candidate Ledger

## Decision

`MIMAP-109A` adds a scalar local-free candidate ledger downstream of
`MIMAP-107A`.

The contract is:

```text
successful released-span ledger report
  -> page / segment / token / block span
  -> record a modeled local-free candidate row
```

This is a ledger-only bridge toward future page-local free-list rows. It does
not execute real segment free and does not mutate a page free-list.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako
```

The owner may:

- consume successful `HakoAllocSegmentAllocationModeledReleasedSpanLedgerReport`
  values;
- validate scalar local-free candidate shape;
- record deterministic `(released-span row index, token)` candidate rows;
- expose scalar read methods and counters;
- reject invalid, source-rejected, duplicate, or unsupported requests.

The owner must not:

- execute real segment allocation/free;
- mutate a free-list;
- mutate page state outside the new scalar candidate ledger;
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
| `0` | recorded local-free candidate |
| `1` | invalid scalar shape |
| `2` | source released-span report rejected or absent |
| `3` | duplicate `(released-span row index, token)` candidate |
| `4` | real segment allocation/free execution requested |
| `5` | free-list mutation requested |
| `6` | raw pointer residence requested |
| `7` | segment-map lookup requested |
| `8` | arena backing allocation requested |
| `9` | atomic bitmap execution requested |
| `10` | OSVM/page-source execution requested |
| `11` | thread/worker execution requested |
| `12` | provider activation requested |
| `13` | backend matcher requested |

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
page state mutation outside the local-free candidate ledger
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
