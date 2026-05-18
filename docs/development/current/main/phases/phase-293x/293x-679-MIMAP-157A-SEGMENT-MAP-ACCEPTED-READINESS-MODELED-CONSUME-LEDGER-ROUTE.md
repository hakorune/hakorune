# 293x-679 MIMAP-157A Segment Map Accepted Readiness Modeled Consume Ledger Route

Status: landed
Date: 2026-05-18

## Decision

Add the first behavior row after the segment-map readiness closeout: consume an
accepted lookup-guarded readiness report into the existing modeled consume /
ledger lane.

## Owner

```text
lang/src/hako_alloc/memory/
apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/
tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
```

## Scope

- Compose MIMAP-153A accepted readiness output with:
  - `HakoAllocSegmentAllocationModeledConsume`,
  - `HakoAllocSegmentAllocationModeledLedger`.
- Record one accepted readiness candidate as one modeled ledger entry.
- Keep validation at L2: VM proof, MIR JSON, and route preflight.
- Defer L3 EXE to the future consume-ledger closeout pack unless a new backend
  route shape appears.

## Stop Lines

- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Added `HakoAllocSegmentMapAcceptedReadinessModeledConsumeLedger` as a thin
  composition owner.
- Added a proof app that routes one accepted MIMAP-153A readiness report into
  MIMAP-091A modeled consume and MIMAP-094A modeled ledger.
- Added L2-only guard coverage: VM proof, MIR JSON assertions, and route
  preflight.

## Closeout

MIMAP-157A landed with this proof surface:

```text
consumed=1,0,0,0,0,0,-1,70,7,2,3,5,3,2,70007002,1,1
rejected=0,1,3,-1,-1,70,7
inactive=0,0,0,0,0,0,0,0,0,0
counts=2,1,1,1,0,0,1,1,70007002
```

The next selected row is `MIMAP-158A`, a diagnostics row for blocked /
duplicate / stale outcomes around the same modeled consume ledger boundary.
Raw pointer residence, real segment-map execution, arena backing, atomic bitmap,
OSVM execution, thread scheduling, provider activation, and backend matchers
remain closed.
