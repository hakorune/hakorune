# 293x-680 MIMAP-158A Segment Map Modeled Consume Ledger Diagnostics

Status: landed
Date: 2026-05-18

## Decision

Add blocked / duplicate / stale diagnostics around the MIMAP-157A modeled
consume ledger boundary.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
apps/hako-alloc-segment-map-accepted-readiness-modeled-consume-ledger-proof/
```

## Scope

- Extend the modeled consume ledger boundary with stable diagnostic counters
  for blocked, duplicate, and stale outcomes.
- Stay on the same L2 validation profile unless a new backend route shape
  appears.
- Prepare a future closeout pack that can carry representative L3 EXE evidence.

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

- Added scalar diagnostic vocabulary to the MIMAP-157A composition owner:
  `ok`, `blocked`, `duplicate`, and `stale`.
- Counted blocked consume rejects, duplicate live-token ledger rejects, and
  stale lookup/readiness rejects.
- Extended the existing proof app and L2 guard; no new EXE evidence is required
  for this row.

## Closeout

MIMAP-158A landed as diagnostics only. It selected:

```text
MIMAP-159A segment-map modeled consume ledger closeout pack
```

Raw pointer residence, real segment-map execution, arena backing, atomic bitmap,
OSVM execution, thread scheduling, provider activation, cross-function `Result`
direct ABI, runtime sum materialization, and backend matchers remain closed.
