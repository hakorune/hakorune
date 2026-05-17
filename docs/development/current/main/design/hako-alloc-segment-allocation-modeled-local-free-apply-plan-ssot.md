---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-111A segment allocation modeled local-free apply plan route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-610-MIMAP-111A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-APPLY-PLAN-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako
  - apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/
---

# Hako Alloc Segment Allocation Modeled Local-Free Apply Plan

## Decision

`MIMAP-111A` adds a scalar local-free apply-plan ledger downstream of
`MIMAP-109A`.

The contract is:

```text
successful local-free candidate report
  -> page / segment / token / block span
  -> record a modeled local-free apply-plan row
```

This is a ledger-only bridge toward future page-local free-list mutation. It
does not execute real segment free, mutate a page free-list, or mutate page
state.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako
```

The owner may:

- consume successful `HakoAllocSegmentAllocationModeledLocalFreeCandidateLedgerReport`
  values;
- validate scalar local-free apply-plan shape;
- record deterministic `(candidate row index, token)` apply-plan rows;
- expose scalar read methods and counters;
- reject invalid, source-rejected, duplicate, or unsupported requests.

The owner must not:

- execute real segment allocation/free;
- mutate a free-list;
- mutate page state outside the new scalar apply-plan ledger;
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
| `0` | recorded local-free apply plan |
| `1` | invalid scalar shape |
| `2` | source local-free candidate report rejected or absent |
| `3` | duplicate `(candidate row index, token)` apply plan |
| `4` | real segment allocation/free execution requested |
| `5` | free-list mutation requested |
| `6` | page-state mutation requested |
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
first=1,0,0,-1,0,60018002,60,18,2,5,3,1,1,1
missing=0,2,-1,1
duplicate=0,3,0,1
recycled=1,0,1,-1,1,60018002,60,18,2,5,3,1,2,2
unsupported=0,5,1
counts=5,2,2,3,0,1,1,0,1
inactive=0,0,0,0,0,0,0,0,0,0,0
summary=ok
```

## Stop Line

This row remains scalar ledger execution. It does not open:

```text
real segment allocation/free
free-list mutation
page-state mutation outside the local-free apply-plan ledger
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
