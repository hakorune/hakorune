---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-094A segment allocation modeled ledger route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-591-MIMAP-094A-SEGMENT-ALLOCATION-MODELED-LEDGER-ROUTE.md
  - lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
  - apps/hako-alloc-segment-allocation-modeled-ledger-proof/
---

# Hako Alloc Segment Allocation Modeled Ledger

## Decision

`MIMAP-094A` adds a scalar modeled allocation ledger after `MIMAP-091A`.

The ledger records accepted modeled consume results as deterministic scalar
rows:

```text
modeled consume accepted
  -> token / segment id / page id / block start / request blocks / page-used facts
  -> local scalar ledger row
```

This is still not a real segment allocation/free route. It is a proof ledger
for later rows that need to find modeled allocation tokens without opening raw
pointer residence, arena backing, atomic bitmap execution, OSVM/page-source
calls, thread scheduling, provider activation, or backend matchers.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako
```

The owner may:

- validate an accepted `MIMAP-091A` modeled consume result;
- verify the scalar arithmetic and token identity;
- append deterministic scalar ledger rows;
- find a live modeled row by token;
- expose scalar read methods and counters.

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
| `0` | accepted modeled ledger row |
| `1` | upstream modeled consume rejected |
| `2` | invalid scalar shape |
| `3` | modeled arithmetic or token mismatch |
| `4` | duplicate modeled allocation token |
| `5` | real segment allocation/free execution requested |
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
first=1,0,0,-1,60018002,60,18,2,3,5,3,2,1,1
second=1,0,1,-1,61019007,61,19,7,1,8,0,7,2,2
finds=0,1,-1
rejects=1,2,3,4,5,6,7,8,9,10,11,12,13
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
