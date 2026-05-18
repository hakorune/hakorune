# 293x-669 MIMAP-149A Segment Allocation Blocked Substrate Matrix Proof

Status: landed
Date: 2026-05-18

## Decision

Add a proof-only matrix that reports the hard substrate blockers for moving
from the current scalar segment allocation model toward real segment
allocation/free.

## Owner

```text
lang/src/hako_alloc/memory/
apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/
```

## Scope

- Compose already-landed scalar facts from:
  - segment allocation readiness,
  - segment/page membership,
  - segment/arena/bitmap boundary inventory.
- Report one stable matrix row for each still-closed blocker:
  - raw pointer residence,
  - segment-map lookup,
  - arena backing allocation,
  - atomic bitmap execution,
  - OSVM execution,
  - real thread scheduling,
  - provider activation,
  - real segment allocation/free execution.
- Preserve the current scalar proof lane as executable in VM and pure-first EXE.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence.
- No segment-map pointer lookup or membership execution.
- No arena backing allocation.
- No atomic bitmap claim/unclaim.
- No page-source or OSVM calls.
- No worker spawning, true scheduling, or source-level concurrency feature.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-149A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation

- Added `HakoAllocSegmentAllocationBlockedSubstrateMatrix` as a proof-only
  owner that composes the existing segment allocation readiness, segment/page
  membership, and segment/arena/bitmap inventory facts.
- Added the MIMAP-149A proof app and guard, including VM, MIR JSON, and
  pure-first EXE verification.
- Registered the owner in the `hako_alloc` module, proof manifest, check
  script index, and allocator memory docs.

## Closeout

MIMAP-149A landed with the blocked-substrate matrix intact:

```text
matrix=0,8,3,255
accepted_reasons=0,0,0
blocker_reasons=2,3,4,3,4,10,5,12
blockers=1,1,1,1,1,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0
```

The next selected row is `MIMAP-150A`, a post-matrix planning row that chooses
which single substrate boundary to open or park next. Real segment
allocation/free, raw pointer residence, segment-map execution, arena backing,
atomic bitmap execution, OSVM execution, thread scheduling, provider
activation, and backend matchers remain closed.
