---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-119A segment allocation modeled local-free integration route.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md
---

# Hako Alloc Segment Allocation Modeled Local-Free Integration SSOT

## Decision

`MIMAP-119A` adds one allocator-owned composition route for the current
modeled local-free chain.

The owner is:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako
```

It composes only existing owners:

```text
successful released-span report
  -> HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger
  -> HakoAllocSegmentAllocationModeledLocalFreeApplyPlan
  -> HakoAllocSegmentAllocationModeledLocalFreePageApply
  -> explicit HakoAllocPageModel.releaseLocal(block_id)
```

The row moves the proof-app hand wiring into an owner boundary. It does not
open segment-map lookup, raw pointer residence, arena backing, atomic bitmap
execution, page-source / OSVM calls, real worker scheduling, or provider
activation.

## Responsibility

Allowed:

- own the composition order;
- require an explicit `HakoAllocPageModel`;
- expose scalar child report row indices and child reason codes;
- expose final page used/local-free/free counters;
- preserve inactive stop-line flags in the integration report.

Forbidden:

- direct page array mutation outside `HakoAllocPageModel.releaseLocal`;
- real segment allocation/free execution beyond the existing page-local model;
- segment-map pointer membership or lookup;
- raw pointer residence;
- arena backing allocation;
- atomic bitmap execution;
- page-source or OSVM execution;
- thread scheduling or worker spawning;
- source-level concurrency feature changes;
- provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`;
- backend `.inc` app/name matcher.

## Validation

```text
bash tools/checks/run_proof_app.sh --only MIMAP-119A
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh
```
